// --------------------------------------------------------------------
// Gufo Agent: Collectors registry
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use crate::config::CollectorConfig;
use common::{AgentError, Collectable, ConfigDiscoveryOpts, ConfigItem, Measure};
use serde::Deserialize;

pub enum Collectors {
    // @@@{{{
    // | {ename}({name}::Collector),
    BlockIo(block_io::Collector),
    Cpu(cpu::Collector),
    Dns(dns::Collector),
    Exec(exec::Collector),
    Fs(fs::Collector),
    Http(http::Collector),
    Memory(memory::Collector),
    ModbusRtu(modbus_rtu::Collector),
    ModbusTcp(modbus_tcp::Collector),
    Network(network::Collector),
    Procstat(procstat::Collector),
    Sockets(sockets::Collector),
    Spool(spool::Collector),
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
            "exec" => Collectors::Exec(from_value(value)?),
            "fs" => Collectors::Fs(from_value(value)?),
            "http" => Collectors::Http(from_value(value)?),
            "memory" => Collectors::Memory(from_value(value)?),
            "modbus_rtu" => Collectors::ModbusRtu(from_value(value)?),
            "modbus_tcp" => Collectors::ModbusTcp(from_value(value)?),
            "network" => Collectors::Network(from_value(value)?),
            "procstat" => Collectors::Procstat(from_value(value)?),
            "sockets" => Collectors::Sockets(from_value(value)?),
            "spool" => Collectors::Spool(from_value(value)?),
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
            Collectors::Exec(_) => exec::Collector::get_name(),
            Collectors::Fs(_) => fs::Collector::get_name(),
            Collectors::Http(_) => http::Collector::get_name(),
            Collectors::Memory(_) => memory::Collector::get_name(),
            Collectors::ModbusRtu(_) => modbus_rtu::Collector::get_name(),
            Collectors::ModbusTcp(_) => modbus_tcp::Collector::get_name(),
            Collectors::Network(_) => network::Collector::get_name(),
            Collectors::Procstat(_) => procstat::Collector::get_name(),
            Collectors::Sockets(_) => sockets::Collector::get_name(),
            Collectors::Spool(_) => spool::Collector::get_name(),
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
            Collectors::Exec(_) => exec::Collector::is_random_offset(),
            Collectors::Fs(_) => fs::Collector::is_random_offset(),
            Collectors::Http(_) => http::Collector::is_random_offset(),
            Collectors::Memory(_) => memory::Collector::is_random_offset(),
            Collectors::ModbusRtu(_) => modbus_rtu::Collector::is_random_offset(),
            Collectors::ModbusTcp(_) => modbus_tcp::Collector::is_random_offset(),
            Collectors::Network(_) => network::Collector::is_random_offset(),
            Collectors::Procstat(_) => procstat::Collector::is_random_offset(),
            Collectors::Sockets(_) => sockets::Collector::is_random_offset(),
            Collectors::Spool(_) => spool::Collector::is_random_offset(),
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
            Collectors::Exec(c) => c.collect().await,
            Collectors::Fs(c) => c.collect().await,
            Collectors::Http(c) => c.collect().await,
            Collectors::Memory(c) => c.collect().await,
            Collectors::ModbusRtu(c) => c.collect().await,
            Collectors::ModbusTcp(c) => c.collect().await,
            Collectors::Network(c) => c.collect().await,
            Collectors::Procstat(c) => c.collect().await,
            Collectors::Sockets(c) => c.collect().await,
            Collectors::Spool(c) => c.collect().await,
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
            "exec".to_string(),
            "fs".to_string(),
            "http".to_string(),
            "memory".to_string(),
            "modbus_rtu".to_string(),
            "modbus_tcp".to_string(),
            "network".to_string(),
            "procstat".to_string(),
            "sockets".to_string(),
            "spool".to_string(),
            "twamp_reflector".to_string(),
            "twamp_sender".to_string(),
            "uptime".to_string(),
            // @@@}}}
        ]
    }
    pub fn discover_config(
        opts: &ConfigDiscoveryOpts,
    ) -> Result<Vec<(&'static str, Vec<ConfigItem>)>, AgentError> {
        let mut r = Vec::new();
        // @@@{{{
        // | if !opts.is_disabled("{name}") {
        // |     let x = {name}::Collector::discover_config(opts)?;
        // |     if !x.is_empty() {
        // |         r.push(({name}::Collector::get_name(), x));
        // |     }
        // | }
        if !opts.is_disabled("block_io") {
            let x = block_io::Collector::discover_config(opts)?;
            if !x.is_empty() {
                r.push((block_io::Collector::get_name(), x));
            }
        }
        if !opts.is_disabled("cpu") {
            let x = cpu::Collector::discover_config(opts)?;
            if !x.is_empty() {
                r.push((cpu::Collector::get_name(), x));
            }
        }
        if !opts.is_disabled("dns") {
            let x = dns::Collector::discover_config(opts)?;
            if !x.is_empty() {
                r.push((dns::Collector::get_name(), x));
            }
        }
        if !opts.is_disabled("exec") {
            let x = exec::Collector::discover_config(opts)?;
            if !x.is_empty() {
                r.push((exec::Collector::get_name(), x));
            }
        }
        if !opts.is_disabled("fs") {
            let x = fs::Collector::discover_config(opts)?;
            if !x.is_empty() {
                r.push((fs::Collector::get_name(), x));
            }
        }
        if !opts.is_disabled("http") {
            let x = http::Collector::discover_config(opts)?;
            if !x.is_empty() {
                r.push((http::Collector::get_name(), x));
            }
        }
        if !opts.is_disabled("memory") {
            let x = memory::Collector::discover_config(opts)?;
            if !x.is_empty() {
                r.push((memory::Collector::get_name(), x));
            }
        }
        if !opts.is_disabled("modbus_rtu") {
            let x = modbus_rtu::Collector::discover_config(opts)?;
            if !x.is_empty() {
                r.push((modbus_rtu::Collector::get_name(), x));
            }
        }
        if !opts.is_disabled("modbus_tcp") {
            let x = modbus_tcp::Collector::discover_config(opts)?;
            if !x.is_empty() {
                r.push((modbus_tcp::Collector::get_name(), x));
            }
        }
        if !opts.is_disabled("network") {
            let x = network::Collector::discover_config(opts)?;
            if !x.is_empty() {
                r.push((network::Collector::get_name(), x));
            }
        }
        if !opts.is_disabled("procstat") {
            let x = procstat::Collector::discover_config(opts)?;
            if !x.is_empty() {
                r.push((procstat::Collector::get_name(), x));
            }
        }
        if !opts.is_disabled("sockets") {
            let x = sockets::Collector::discover_config(opts)?;
            if !x.is_empty() {
                r.push((sockets::Collector::get_name(), x));
            }
        }
        if !opts.is_disabled("spool") {
            let x = spool::Collector::discover_config(opts)?;
            if !x.is_empty() {
                r.push((spool::Collector::get_name(), x));
            }
        }
        if !opts.is_disabled("twamp_reflector") {
            let x = twamp_reflector::Collector::discover_config(opts)?;
            if !x.is_empty() {
                r.push((twamp_reflector::Collector::get_name(), x));
            }
        }
        if !opts.is_disabled("twamp_sender") {
            let x = twamp_sender::Collector::discover_config(opts)?;
            if !x.is_empty() {
                r.push((twamp_sender::Collector::get_name(), x));
            }
        }
        if !opts.is_disabled("uptime") {
            let x = uptime::Collector::discover_config(opts)?;
            if !x.is_empty() {
                r.push((uptime::Collector::get_name(), x));
            }
        }
        // @@@}}}
        Ok(r)
    }
}
