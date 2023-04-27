// ---------------------------------------------------------------------
// Constant bitrate Packet Model
// ---------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// See LICENSE for details
// ---------------------------------------------------------------------

use super::{GetPacket, Packet, NS};
use common::AgentError;
use serde::Deserialize;
use std::hash::Hash;

#[derive(Deserialize, Debug, Clone, Hash)]
pub struct Config {
    #[serde(rename = "model_bandwidth")]
    pub bandwidth: usize,
    #[serde(rename = "model_size")]
    pub size: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct CbrModel {
    size: usize,
    next_ns: u64,
}

impl TryFrom<serde_yaml::Value> for CbrModel {
    type Error = AgentError;

    fn try_from(value: serde_yaml::Value) -> Result<Self, Self::Error> {
        let cfg = serde_yaml::from_value::<Config>(value)
            .map_err(|e| AgentError::ConfigurationError(e.to_string()))?;
        Ok(Self {
            size: cfg.size,
            next_ns: NS / (cfg.bandwidth / (cfg.size * 8)) as u64,
        })
    }
}

impl GetPacket for CbrModel {
    fn get_packet(&self, seq: u64) -> Packet {
        Packet {
            seq,
            size: self.size,
            next_ns: self.next_ns,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proto::pktmodel::{GetPacket, ModelConfig, PacketModels};

    #[test]
    fn test_cbr_model() {
        let model = PacketModels::try_from(ModelConfig::Cbr(CbrModelConfig {
            bandwidth: 8_000_000,
            size: 100,
        }))
        .unwrap();
        let pkt = model.get_packet(0);
        let expected = Packet {
            seq: 0,
            size: 100,
            next_ns: 100_000,
        };
        assert_eq!(pkt, expected);
    }
}
