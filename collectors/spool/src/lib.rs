// --------------------------------------------------------------------
// Gufo Agent: spool collector implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use async_trait::async_trait;
use common::{counter, AgentError, Collectable, Measure};
use openmetrics::{parse, ParseConfig};
use serde::{Deserialize, Serialize};
use std::fs::{metadata, read_dir, read_to_string, remove_file};
use std::path::Path;

// Collector config
#[derive(Deserialize, Serialize)]
pub struct Config {
    path: String,
    #[serde(default = "default_false")]
    dry_run: bool,
    #[serde(default = "default_false")]
    trust_timestamps: bool,
}

// Collector structure
pub struct Collector {
    path: String,
    dry_run: bool,
    parse_cfg: ParseConfig,
    // Stats
    spool_jobs: u64,
    spool_jobs_success: u64,
    spool_jobs_failed: u64,
    spool_parsed: u64,
}

// Generated metrics
counter!(spool_jobs, "Total spool jobs processed", path);
counter!(
    spool_jobs_success,
    "Spool jobs processed successfully",
    path
);
counter!(spool_jobs_failed, "Spool jobs failed to process", path);
counter!(spool_parsed, "Parsed metric items", path);

// Instantiate collector from given config
impl TryFrom<Config> for Collector {
    type Error = AgentError;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        Ok(Self {
            path: value.path,
            dry_run: value.dry_run,
            parse_cfg: ParseConfig {
                trust_timestamps: value.trust_timestamps,
            },
            spool_jobs: 0,
            spool_jobs_success: 0,
            spool_jobs_failed: 0,
            spool_parsed: 0,
        })
    }
}

// Collector implementation
#[async_trait]
impl Collectable for Collector {
    const NAME: &'static str = "spool";
    type Config = Config;

    async fn collect(&mut self) -> Result<Vec<Measure>, AgentError> {
        // Collect data
        log::info!("Scanning directory {}", self.path);
        // List directory
        let mut paths = Vec::new();
        for rd in read_dir(&self.path).map_err(|e| AgentError::InternalError(e.to_string()))? {
            match rd {
                Ok(e) => paths.push(e),
                Err(e) => {
                    log::error!("Cannot list file: {}", e);
                    continue;
                }
            }
        }
        // Sort result
        paths.sort_by_key(|dir| dir.path());
        //
        let mut r = vec![];
        for entry in paths {
            self.spool_jobs += 1;
            match self.process_file(entry.path().as_path()) {
                Ok(mut sr) => {
                    self.spool_jobs_success += 1;
                    self.spool_parsed += sr.len() as u64;
                    r.append(&mut sr);
                }
                Err(e) => {
                    log::error!("Failed to process {}: {}", entry.path().display(), e);
                    self.spool_jobs_failed += 1;
                }
            }
        }
        // Apply internal metrics
        r.push(spool_jobs(self.spool_jobs, self.path.clone()));
        r.push(spool_jobs_success(
            self.spool_jobs_success,
            self.path.clone(),
        ));
        r.push(spool_jobs_failed(self.spool_jobs_failed, self.path.clone()));
        r.push(spool_parsed(self.spool_parsed, self.path.clone()));
        // Push result
        Ok(r)
    }
}

impl Collector {
    fn process_file(&self, path: &Path) -> Result<Vec<Measure>, AgentError> {
        log::debug!("Processing {:?}", path);
        // Check file is regular file
        let meta = metadata(path)?;
        if !meta.is_file() {
            return Err(AgentError::InternalError("not a regular file".to_string()));
        }
        //
        let data = read_to_string(path)?;
        let parsed = parse(data.as_str(), &self.parse_cfg)?;
        if !self.dry_run {
            log::debug!("Removing {:?}", path);
            if let Err(e) = remove_file(path) {
                log::error!("Cannot remove file {:?}: {}", path, e);
            }
        }
        Ok(parsed)
    }
}

fn default_false() -> bool {
    false
}
