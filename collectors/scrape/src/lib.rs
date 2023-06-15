// --------------------------------------------------------------------
// Gufo Agent: scrape collector implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

mod sd;

use async_trait::async_trait;
use common::{
    AgentError, AgentResult, Collectable, ConfigDiscoveryOpts, ConfigItem, Label, Measure,
};
use openmetrics::ParsedMetrics;
use sd::{Sd, SdConfig, ServiceDiscovery};
use serde::{Deserialize, Serialize};

// Collector config
#[derive(Deserialize, Serialize)]
pub struct Config {
    service_discovery: SdConfig,
}

// Collector structure
pub struct Collector {
    sd: Sd,
}

// Generated metrics

// Instantiate collector from given config
impl TryFrom<Config> for Collector {
    type Error = AgentError;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        Ok(Self {
            sd: Sd::try_from(value.service_discovery)?,
        })
    }
}

// Collector implementation
#[async_trait]
impl Collectable for Collector {
    const NAME: &'static str = "scrape";
    type Config = Config;

    async fn collect(&mut self) -> AgentResult<Vec<Measure>> {
        // Perform service discovery
        let services = self.sd.get_services()?;
        if services.is_empty() {
            log::info!("No services discovered, exiting");
            return Ok(Vec::default());
        }
        // Collect data
        let mut r = Vec::default();
        let client = reqwest::Client::builder()
            .gzip(true)
            .build()
            .map_err(|e| AgentError::InternalError(e.to_string()))?;
        // @todo: Concurrency
        for item in services.iter() {
            let url = format!("http://{}/metrics", item.target);
            let address_label = Label::new("__address__", item.target.to_owned());
            match client.get(&url).send().await {
                Ok(resp) => match resp.text().await {
                    Ok(data) => match ParsedMetrics::try_from(data.as_str()) {
                        Ok(mut parsed) => {
                            // Install virtual labels
                            for item in parsed.0.iter_mut() {
                                item.labels.push(address_label.to_owned());
                            }
                            r.append(&mut parsed.0);
                        }
                        Err(e) => log::error!("Failed to parse from {}: {}", url, e),
                    },
                    Err(e) => log::error!("Failed to fetch {}: {}", url, e),
                },
                Err(e) => log::error!("Failed to fetch {}: {}", url, e),
            }
        }
        // Push result
        Ok(r)
    }
    // !!! Uncomment for config discovery
    // fn discover_config(_: &ConfigDiscoveryOpts) -> Result<Vec<ConfigItem>, AgentError> {
    //     let cfg = Config;
    //     Ok(vec![ConfigItem::from_config(cfg)?])
    // }
}
