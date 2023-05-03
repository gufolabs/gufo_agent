// ---------------------------------------------------------------------
// IMix packet model
// ---------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// See LICENSE for details
// ---------------------------------------------------------------------

use super::{GetPacket, Packet, NS};
use common::AgentError;
use serde::Deserialize;
use std::convert::TryFrom;
use std::hash::Hash;

#[derive(Deserialize, Debug, Clone, Hash)]
pub struct Config {
    pub bandwidth: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct ImixModel {
    next_ns: u64,
}

impl TryFrom<serde_yaml::Value> for ImixModel {
    type Error = AgentError;

    fn try_from(value: serde_yaml::Value) -> Result<Self, Self::Error> {
        let cfg = serde_yaml::from_value::<Config>(value)
            .map_err(|e| AgentError::ConfigurationError(e.to_string()))?;
        Ok(Self {
            next_ns: NS * IMIX_ROUND / (cfg.bandwidth * IMIX_SAMPLE_COUNT) as u64,
        })
    }
}

impl GetPacket for ImixModel {
    fn get_packet(&self, seq: u64) -> Packet {
        Packet {
            seq,
            size: IMIX_SAMPLE[seq as usize % IMIX_SAMPLE_COUNT],
            next_ns: self.next_ns,
        }
    }
}

/// IMIX iterator.
/// Simple IMIX model consists of 7 packets of 64 octets, 4 of 576 and one for 1500.
/// As TWAMP-Response is 20+8+41, pad small packets to 70 octets.  
const IMIX1: usize = 70;
const IMIX1_COUNT: usize = 7;
const IMIX2: usize = 576;
const IMIX2_COUNT: usize = 4;
const IMIX3: usize = 1500;
const IMIX3_COUNT: usize = 1;
const IMIX_SAMPLE_COUNT: usize = IMIX1_COUNT + IMIX2_COUNT + IMIX3_COUNT;
static IMIX_SAMPLE: &[usize; 12] = &[
    IMIX1, IMIX2, IMIX1, IMIX2, IMIX1, IMIX2, IMIX1, IMIX2, IMIX1, IMIX1, IMIX1, IMIX3,
];
const IMIX_ROUND: u64 =
    ((IMIX1_COUNT * IMIX1 + IMIX2_COUNT * IMIX2 + IMIX3_COUNT * IMIX3) * 8) as u64;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pkt::{GetPacket, PacketModel};
    use crate::Config;

    #[test]
    fn test_imix_model() {
        let yaml = r###"
        reflector: "127.0.0.1"
        n_packets: 100
        model: imix
        bandwidth: 12252000
        "###;
        let cfg = serde_yaml::from_str::<Config>(yaml).unwrap();
        let model = PacketModel::try_from(cfg).unwrap();
        for seq in 0..IMIX_SAMPLE_COUNT {
            let s = seq as u64;
            let pkt = model.get_packet(s);
            let expected = Packet {
                seq: s,
                size: IMIX_SAMPLE[seq % IMIX_SAMPLE_COUNT],
                next_ns: 233_648,
            };
            assert_eq!(pkt, expected);
        }
    }
}
