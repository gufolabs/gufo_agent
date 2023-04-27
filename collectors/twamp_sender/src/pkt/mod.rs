// --------------------------------------------------------------------
// Gufo Agent: Packet Models
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

mod cbr;
mod g711;
mod g729;
mod imix;
use super::Config;
use cbr::CbrModel;
use common::AgentError;
use g711::G711Model;
use g729::G729Model;
use imix::ImixModel;

/// Packet for modeling
#[derive(Debug, PartialEq)]
pub struct Packet {
    pub seq: u64,
    pub size: usize,
    pub next_ns: u64,
}

// Nanosecond
pub(crate) const NS: u64 = 1_000_000_000;

#[derive(Clone)]
pub enum PacketModel {
    G711(G711Model),
    G729(G729Model),
    Cbr(CbrModel),
    Imix(ImixModel),
}

pub trait GetPacket {
    fn get_packet(&self, seq: u64) -> Packet;
}

impl GetPacket for PacketModel {
    fn get_packet(&self, seq: u64) -> Packet {
        match self {
            PacketModel::G711(model) => model.get_packet(seq),
            PacketModel::G729(model) => model.get_packet(seq),
            PacketModel::Cbr(model) => model.get_packet(seq),
            PacketModel::Imix(model) => model.get_packet(seq),
        }
    }
}

impl TryFrom<Config> for PacketModel {
    type Error = AgentError;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        match value.model.as_str() {
            "g711" => Ok(PacketModel::G711(G711Model::try_from(value.model_config)?)),
            "g729" => Ok(PacketModel::G729(G729Model::try_from(value.model_config)?)),
            "cbr" => Ok(PacketModel::Cbr(CbrModel::try_from(value.model_config)?)),
            "imix" => Ok(PacketModel::Imix(ImixModel::try_from(value.model_config)?)),
            _ => Err(AgentError::ConfigurationError(format!(
                "Invalid model: {}",
                value.model
            ))),
        }
    }
}
