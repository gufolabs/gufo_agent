// ---------------------------------------------------------------------
// G.729 Packet model
// ---------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// See LICENSE for details
// ---------------------------------------------------------------------

use super::{GetPacket, Packet, NS};
use common::AgentError;
use emodel::{CodecEModel, G729_CODEC_EMODEL};
use serde::Deserialize;
use std::convert::TryFrom;
use std::hash::Hash;

#[derive(Deserialize, Debug, Clone, Hash)]
pub struct Config {}

#[derive(Debug, Copy, Clone)]
pub struct G729Model;

impl TryFrom<serde_yaml::Value> for G729Model {
    type Error = AgentError;

    fn try_from(value: serde_yaml::Value) -> Result<Self, Self::Error> {
        let _cfg = serde_yaml::from_value::<Config>(value)
            .map_err(|e| AgentError::ConfigurationError(e.to_string()))?;
        Ok(Self)
    }
}

impl GetPacket for G729Model {
    fn get_packet(&self, seq: u64) -> Packet {
        Packet {
            seq,
            size: 20 + 8 + 12 + 20,
            next_ns: NS / 50,
        }
    }
    fn get_emodel(&self) -> Option<&'static CodecEModel> {
        Some(&G729_CODEC_EMODEL)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pkt::{GetPacket, PacketModel};
    use crate::Config;

    #[test]
    fn test_g729_model() {
        let yaml = r###"
        reflector: "127.0.0.1"
        n_packets: 100
        model: g729
        "###;
        let cfg = serde_yaml::from_str::<Config>(yaml).unwrap();
        let model = PacketModel::try_from(cfg).unwrap();
        let pkt = model.get_packet(0);
        let expected = Packet {
            seq: 0,
            size: 60,
            next_ns: 20_000_000,
        };
        assert_eq!(pkt, expected);
    }
}
