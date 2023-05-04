// --------------------------------------------------------------------
// Gufo Agent: sockets collector implementation
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
gauge!(tcp4_sockets_used, "Total amount of IPv4 TCP sockets used");
gauge!(tcp6_sockets_used, "Total amount of IPv6 TCP sockets used");
gauge!(udp4_sockets_used, "Total amount of IPv4 UDP sockets used");
gauge!(udp6_sockets_used, "Total amount of IPv6 UDP sockets used");

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
    const NAME: &'static str = "sockets";
    type Config = Config;

    async fn collect(&mut self) -> Result<Vec<Measure>, AgentError> {
        // Collect data
        let stats = System::new()
            .socket_stats()
            .map_err(|e| AgentError::InternalError(e.to_string()))?;
        // Push result
        Ok(vec![
            tcp4_sockets_used(stats.tcp_sockets_in_use as u64),
            tcp6_sockets_used(stats.tcp6_sockets_in_use as u64),
            udp4_sockets_used(stats.udp_sockets_in_use as u64),
            udp6_sockets_used(stats.udp6_sockets_in_use as u64),
        ])
    }
    fn discover_config(_: &ConfigDiscoveryOpts) -> Result<Vec<ConfigItem>, AgentError> {
        let cfg = Config;
        Ok(vec![ConfigItem::from_config(cfg)?])
    }
}
