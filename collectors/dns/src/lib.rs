// --------------------------------------------------------------------
// Gufo Agent: dns collector implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use async_trait::async_trait;
use common::{counter, gauge, AgentError, Collectable, Measure, Timing};
use serde::Deserialize;
use std::str::FromStr;
use std::time::Instant;
use trust_dns_proto::rr::record_type::RecordType;
use trust_dns_resolver::TokioAsyncResolver;

// Collector config
#[derive(Deserialize)]
pub struct Config {
    pub query: String,
    #[serde(default = "default_type_a")]
    pub query_type: String,
    #[serde(default = "default_one")]
    pub n: u64,
}

// Collector structure
pub struct Collector {
    query: String,
    record_type: RecordType,
    n: u64,
    // State
    total: u64,
    success: u64,
    failed: u64,
}

// Generated metrics
counter!(requests_total, "Total DNS requests performed", query, type);
counter!(requests_success, "Successful DNS requests", query, type);
counter!(requests_failed, "Failed DNS requests", query, type);
gauge!(
    min_ns,
    "Minimal response delay of the serie in nanoseconds",
    query,
    type
);
gauge!(
    max_ns,
    "Maximal response delay of the serie in nanoseconds",
    query,
    type
);
gauge!(
    avg_ns,
    "Average response delay of the serie in nanoseconds",
    query,
    type
);
gauge!(
    jitter_ns,
    "Jitter of the response delay of the serie in nanoseconds",
    query,
    type
);

// Instantiate collector from given config
impl TryFrom<Config> for Collector {
    type Error = AgentError;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        Ok(Self {
            query: value.query,
            record_type: RecordType::from_str(&value.query_type)
                .map_err(|e| AgentError::ConfigurationError(e.to_string()))?,
            n: value.n,
            total: 0,
            success: 0,
            failed: 0,
        })
    }
}

// Collector implementation
#[async_trait]
impl Collectable for Collector {
    const NAME: &'static str = "dns";
    type Config = Config;

    async fn collect(&mut self) -> Result<Vec<Measure>, AgentError> {
        // Prepare resolver
        let resolver = TokioAsyncResolver::tokio_from_system_conf()
            .map_err(|e| AgentError::ConfigurationError(e.to_string()))?;
        // Send serie of requests and measure timings
        let mut success = 0u64;
        let mut timing = Timing::default();
        for i in 0..self.n {
            log::debug!(
                "[{}/{}] Lookup {} @{}",
                i + 1,
                self.n,
                self.query,
                self.record_type
            );
            let t0 = Instant::now();
            match resolver.lookup(self.query.clone(), self.record_type).await {
                Ok(_) => {
                    success += 1;
                }
                Err(e) => {
                    log::error!("Failed to resolve: {}", e);
                }
            }
            timing.apply(t0.elapsed().as_nanos() as i64);
        }
        timing.done();
        // Update state
        self.total += self.n;
        self.success += success;
        self.failed += self.n - success;
        // Push result
        Ok(vec![
            requests_total(self.total, self.query.clone(), self.record_type),
            requests_success(self.success, self.query.clone(), self.record_type),
            requests_failed(self.failed, self.query.clone(), self.record_type),
            min_ns(timing.min_ns, self.query.clone(), self.record_type),
            max_ns(timing.max_ns, self.query.clone(), self.record_type),
            avg_ns(timing.avg_ns, self.query.clone(), self.record_type),
            jitter_ns(timing.jitter_ns, self.query.clone(), self.record_type),
        ])
    }
}

// Default record type for config
fn default_type_a() -> String {
    "A".into()
}

// Default count for config
fn default_one() -> u64 {
    1
}
