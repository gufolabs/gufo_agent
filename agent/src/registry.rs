// --------------------------------------------------------------------
// Gufo Agent: Collectors registry
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use crate::config::CollectorConfig;
use common::{AgentError, Collectable, Measure};
use serde::Deserialize;

pub enum Collectors {
    // @@@{{{
    // | {ename}({name}::Collector),
    BlockIo(block_io::Collector),
    Cpu(cpu::Collector),
    Dns(dns::Collector),
    Fs(fs::Collector),
    Memory(memory::Collector),
    Network(network::Collector),
    TwampReflector(twamp_reflector::Collector),
    TwampSender(twamp_sender::Collector),
    Uptime(uptime::Collector),
    // @@@}}}
}

fn _clean_value(value: serde_yaml::Value) -> serde_yaml::Value {
    match value {
        serde_yaml::Value::Mapping(ref m) => {
            if m.is_empty() {
                serde_yaml::Value::Null
            } else {
                value
            }
        }
        _ => value,
    }
}

fn from_value<T>(value: CollectorConfig) -> Result<T, AgentError>
where
    T: Collectable,
    AgentError: From<<T as TryFrom<<T as Collectable>::Config>>::Error>,
    for<'de> <T as Collectable>::Config: Deserialize<'de>,
{
    let cv = _clean_value(value.config);
    let cfg = serde_yaml::from_value::<<T as Collectable>::Config>(cv)
        .map_err(|e| AgentError::ConfigurationError(e.to_string()))?;
    let collector = T::try_from(cfg)?;
    Ok(collector)
}

impl TryFrom<CollectorConfig> for Collectors {
    type Error = AgentError;

    fn try_from(value: CollectorConfig) -> Result<Self, Self::Error> {
        Ok(match value.r#type.as_str() {
            // @@@{{{
            // | "{name}" => Collectors::{ename}(from_value(value)?),
            "block_io" => Collectors::BlockIo(from_value(value)?),
            "cpu" => Collectors::Cpu(from_value(value)?),
            "dns" => Collectors::Dns(from_value(value)?),
            "fs" => Collectors::Fs(from_value(value)?),
            "memory" => Collectors::Memory(from_value(value)?),
            "network" => Collectors::Network(from_value(value)?),
            "twamp_reflector" => Collectors::TwampReflector(from_value(value)?),
            "twamp_sender" => Collectors::TwampSender(from_value(value)?),
            "uptime" => Collectors::Uptime(from_value(value)?),
            // @@@}}}
            _ => return Err(AgentError::InvalidCollectorError(value.r#type.clone())),
        })
    }
}

impl Collectors {
    pub fn get_name(&self) -> &'static str {
        match self {
            // @@@{{{
            // | Collectors::{ename}(_) => {name}::Collector::get_name(),
            Collectors::BlockIo(_) => block_io::Collector::get_name(),
            Collectors::Cpu(_) => cpu::Collector::get_name(),
            Collectors::Dns(_) => dns::Collector::get_name(),
            Collectors::Fs(_) => fs::Collector::get_name(),
            Collectors::Memory(_) => memory::Collector::get_name(),
            Collectors::Network(_) => network::Collector::get_name(),
            Collectors::TwampReflector(_) => twamp_reflector::Collector::get_name(),
            Collectors::TwampSender(_) => twamp_sender::Collector::get_name(),
            Collectors::Uptime(_) => uptime::Collector::get_name(),
            // @@@}}}
        }
    }

    pub fn is_random_offset(&self) -> bool {
        match self {
            // @@@{{{
            // | Collectors::{ename}(_) => {name}::Collector::is_random_offset(),
            Collectors::BlockIo(_) => block_io::Collector::is_random_offset(),
            Collectors::Cpu(_) => cpu::Collector::is_random_offset(),
            Collectors::Dns(_) => dns::Collector::is_random_offset(),
            Collectors::Fs(_) => fs::Collector::is_random_offset(),
            Collectors::Memory(_) => memory::Collector::is_random_offset(),
            Collectors::Network(_) => network::Collector::is_random_offset(),
            Collectors::TwampReflector(_) => twamp_reflector::Collector::is_random_offset(),
            Collectors::TwampSender(_) => twamp_sender::Collector::is_random_offset(),
            Collectors::Uptime(_) => uptime::Collector::is_random_offset(),
            // @@@}}}
        }
    }

    pub async fn collect(&mut self) -> Result<Vec<Measure>, AgentError> {
        match self {
            // @@@{{{
            // | Collectors::{ename}(c) => c.collect().await,
            Collectors::BlockIo(c) => c.collect().await,
            Collectors::Cpu(c) => c.collect().await,
            Collectors::Dns(c) => c.collect().await,
            Collectors::Fs(c) => c.collect().await,
            Collectors::Memory(c) => c.collect().await,
            Collectors::Network(c) => c.collect().await,
            Collectors::TwampReflector(c) => c.collect().await,
            Collectors::TwampSender(c) => c.collect().await,
            Collectors::Uptime(c) => c.collect().await,
            // @@@}}}
        }
    }
    pub fn to_vec() -> Vec<String> {
        vec![
            // @@@{{{
            // | "{name}".to_string(),
            "block_io".to_string(),
            "cpu".to_string(),
            "dns".to_string(),
            "fs".to_string(),
            "memory".to_string(),
            "network".to_string(),
            "twamp_reflector".to_string(),
            "twamp_sender".to_string(),
            "uptime".to_string(),
            // @@@}}}
        ]
    }
}
