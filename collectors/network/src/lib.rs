// --------------------------------------------------------------------
// Gufo Agent: network collector implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use async_trait::async_trait;
use common::{counter, AgentError, Collectable, ConfigDiscoveryOpts, ConfigItem, Measure};
use serde::{Deserialize, Serialize};
use systemstat::{Platform, System};

// Collector config
#[derive(Deserialize, Serialize)]
pub struct Config;

// Collector structure
pub struct Collector;

// Generated metrics
counter!(rx_octets, "Total number of octets received", iface);
counter!(tx_octets, "Total number of octets sent", iface);
counter!(rx_packets, "Total number of packets received", iface);
counter!(tx_packets, "Total number of packets sent", iface);
counter!(rx_errors, "Total number of receive errors", iface);
counter!(tx_errors, "Total number of transmit errors", iface);

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
    const NAME: &'static str = "network";
    type Config = Config;

    async fn collect(&mut self) -> Result<Vec<Measure>, AgentError> {
        // Collect data
        let sys = System::new();
        let interfaces = sys
            .networks()
            .map_err(|e| AgentError::InternalError(e.to_string()))?;
        let mut r = Vec::with_capacity(interfaces.len() * 6);
        for iface in interfaces.values() {
            let stats = sys
                .network_stats(&iface.name)
                .map_err(|e| AgentError::InternalError(e.to_string()))?;
            r.push(rx_octets(stats.rx_bytes.as_u64(), &iface.name));
            r.push(tx_octets(stats.tx_bytes.as_u64(), &iface.name));
            r.push(rx_packets(stats.rx_packets, &iface.name));
            r.push(tx_packets(stats.tx_packets, &iface.name));
            r.push(rx_errors(stats.rx_errors, &iface.name));
            r.push(tx_errors(stats.tx_errors, &iface.name));
        }
        // Push result
        Ok(r)
    }
    fn discover_config(_: &ConfigDiscoveryOpts) -> Result<Vec<ConfigItem>, AgentError> {
        let cfg = Config;
        Ok(vec![ConfigItem::from_config(cfg)?])
    }
}
