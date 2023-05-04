// --------------------------------------------------------------------
// Gufo Agent: fs collector implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use async_trait::async_trait;
use common::{gauge, AgentError, Collectable, ConfigDiscoveryOpts, ConfigItem, Measure};
use serde::{Deserialize, Serialize};
use systemstat::{Platform, System};

// Collector config
#[derive(Deserialize, Serialize)]
pub struct Config;

// Collector structure
pub struct Collector;

// Generated metrics
gauge!(files, "Inodes used", mount, type);
gauge!(files_total, "Total inodes count", mount, type);
gauge!(files_available, "Inodes available", mount, type);
gauge!(free, "Free disk space, bytes", mount, type);
gauge!(total, "Total disk space, bytes", mount, type);
gauge!(available, "Available disk space, bytes", mount, type);

// Instantiate collector from given config
impl TryFrom<Config> for Collector {
    type Error = AgentError;

    fn try_from(_: Config) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

// Collector implementation
#[async_trait]
impl Collectable for Collector {
    const NAME: &'static str = "fs";
    type Config = Config;

    async fn collect(&mut self) -> Result<Vec<Measure>, AgentError> {
        // Collect data
        let mounts = System::new()
            .mounts()
            .map_err(|e| AgentError::InternalError(e.to_string()))?;
        // Build result
        let mut r = Vec::with_capacity(mounts.len() * 6);
        for fs in mounts.iter() {
            if !self.is_ignored_fs(&fs.fs_type, &fs.fs_mounted_on) {
                r.push(files(fs.files as u64, &fs.fs_mounted_on, &fs.fs_type));
                r.push(files_total(
                    fs.files_total as u64,
                    &fs.fs_mounted_on,
                    &fs.fs_type,
                ));
                r.push(files_available(
                    fs.files_avail as u64,
                    &fs.fs_mounted_on,
                    &fs.fs_type,
                ));
                r.push(free(fs.free.as_u64(), &fs.fs_mounted_on, &fs.fs_type));
                r.push(available(fs.avail.as_u64(), &fs.fs_mounted_on, &fs.fs_type));
                r.push(total(fs.total.as_u64(), &fs.fs_mounted_on, &fs.fs_type));
            }
        }
        // Push result
        Ok(r)
    }
    fn discover_config(_: &ConfigDiscoveryOpts) -> Result<Vec<ConfigItem>, AgentError> {
        let cfg = Config;
        Ok(vec![ConfigItem::from_config(cfg)?])
    }
}

impl Collector {
    // Check if filesystem must be ignored
    fn is_ignored_fs(&self, fs_type: &str, mounted_on: &str) -> bool {
        self.is_ignored_fs_type(fs_type) || self.is_ignored_fs_mount(mounted_on)
    }

    #[cfg(target_os = "linux")]
    fn is_ignored_fs_type(&self, fs_type: &str) -> bool {
        matches!(fs_type, "proc" | "devpts" | "sysfs" | "cgroup" | "overlay")
    }
    #[cfg(not(target_os = "linux"))]
    fn is_ignored_fs_type(&self, fs_type: &str) -> bool {
        false
    }
    #[cfg(target_os = "linux")]
    fn is_ignored_fs_mount(&self, mounted_on: &str) -> bool {
        matches!(
            mounted_on,
            "/proc" | "/proc/" | "/dev" | "/dev/" | "/sys" | "/sys/"
        ) || mounted_on.starts_with("/proc/")
            || mounted_on.starts_with("/dev/")
            || mounted_on.starts_with("/sys/")
    }
    #[cfg(not(target_os = "linux"))]
    fn is_ignored_fs_mount(&self, mounted_on: &str) -> bool {
        false
    }
}
