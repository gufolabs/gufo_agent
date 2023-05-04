// --------------------------------------------------------------------
// Gufo Agent: http collector implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use async_trait::async_trait;
use common::{gauge, AgentError, Collectable, Measure};
use serde::Deserialize;
use std::time::Instant;

// Collector config
#[derive(Deserialize)]
pub struct Config {
    pub url: String,
}

// Collector structure
pub struct Collector {
    url: String,
}

// Generated metrics
gauge!(time_ns, "Response time in nanoseconds");
gauge!(bytes, "Response size in bytes");
gauge!(compressed_bytes, "Compressed response size in bytes");

// Instantiate collector from given config
impl TryFrom<Config> for Collector {
    type Error = AgentError;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        Ok(Self { url: value.url })
    }
}

// Collector implementation
#[async_trait]
impl Collectable for Collector {
    const NAME: &'static str = "http";
    type Config = Config;

    async fn collect(&mut self) -> Result<Vec<Measure>, AgentError> {
        // Collect data
        let t0 = Instant::now();
        let client = reqwest::Client::builder()
            .gzip(true)
            .build()
            .map_err(|e| AgentError::InternalError(e.to_string()))?;
        let mut resp = client
            .get(&self.url)
            .send()
            .await
            .map_err(|e| AgentError::InternalError(e.to_string()))?;
        let mut uncompressed = 0u64;
        while let Some(chunk) = resp
            .chunk()
            .await
            .map_err(|e| AgentError::InternalError(e.to_string()))?
        {
            uncompressed += chunk.len() as u64;
        }
        // Return result
        Ok(vec![
            time_ns(t0.elapsed().as_nanos() as u64),
            bytes(uncompressed),
            compressed_bytes(match &resp.content_length() {
                Some(x) => *x,
                None => uncompressed,
            }),
        ])
    }
}
