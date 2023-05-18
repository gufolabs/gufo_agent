// --------------------------------------------------------------------
// Gufo Agent: modbus_tcp collector implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use async_trait::async_trait;
use common::{AgentError, Collectable, Measure};
use common::{Labels, LabelsConfig};
use modbus::{ModbusFormat, RegisterType};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::time::{timeout, Duration};
use tokio_modbus::{client::tcp::connect_slave, prelude::Reader, Address, Quantity, Slave};

// Collector config
#[derive(Deserialize, Serialize)]
pub struct Config {
    pub address: String,
    #[serde(default = "default_502")]
    pub port: u16,
    #[serde(default = "default_5000")]
    pub timeout_ms: u64,
    items: Vec<CollectorConfigItem>,
}

#[derive(Deserialize, Serialize)]
pub struct CollectorConfigItem {
    pub name: String,
    pub help: String,
    pub labels: LabelsConfig,
    pub register: u16,
    #[serde(default = "default_holding")]
    pub register_type: RegisterType,
    pub format: ModbusFormat,
    #[serde(default = "default_255")]
    pub slave: u8,
}

// Collector structure
pub struct Collector {
    sock_addr: SocketAddr,
    timeout_ms: u64,
    items: Vec<CollectorItem>,
}

struct CollectorItem {
    name: String,
    help: String,
    labels: Labels,
    register: Address,
    count: Quantity,
    register_type: RegisterType,
    format: ModbusFormat,
    slave: Slave,
}

// Instantiate collector from given config
impl TryFrom<Config> for Collector {
    type Error = AgentError;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        // Parse address
        let sock_addr = format!("{}:{}", value.address, value.port)
            .parse()
            .map_err(|_| AgentError::ParseError("cannot parse address".to_string()))?;
        //
        Ok(Self {
            sock_addr,
            timeout_ms: value.timeout_ms,
            items: value
                .items
                .iter()
                .map(|x| CollectorItem {
                    name: x.name.clone(),
                    help: x.help.clone(),
                    labels: x.labels.clone().into(),
                    register: x.register,
                    count: x.format.min_count(),
                    register_type: x.register_type.clone(),
                    format: x.format,
                    slave: Slave::from(x.slave),
                })
                .collect(),
        })
    }
}

// Collector implementation
#[async_trait]
impl Collectable for Collector {
    const NAME: &'static str = "modbus_tcp";
    type Config = Config;

    async fn collect(&mut self) -> Result<Vec<Measure>, AgentError> {
        // Connect
        // @todo: timeout?
        let duration = Duration::from_millis(self.timeout_ms);
        // Collect data
        let mut r = Vec::with_capacity(self.items.len());
        for item in self.items.iter() {
            let mut ctx = connect_slave(self.sock_addr, item.slave)
                .await
                .map_err(|e| AgentError::InternalError(e.to_string()))?;
            // Read result
            let data = match item.register_type {
                RegisterType::Holding => timeout(
                    duration,
                    ctx.read_holding_registers(item.register, item.count),
                )
                .await?
                .map_err(|e| AgentError::InternalError(e.to_string()))?,
                RegisterType::Input => timeout(
                    duration,
                    ctx.read_input_registers(item.register, item.count),
                )
                .await?
                .map_err(|e| AgentError::InternalError(e.to_string()))?,
                RegisterType::Coil => timeout(duration, ctx.read_coils(item.register, item.count))
                    .await?
                    .map_err(|e| AgentError::InternalError(e.to_string()))?
                    .iter()
                    .map(|v| if *v { 1 } else { 0 })
                    .collect(),
            };
            // Decode value
            let value = match item.format.modbus_try_from(data) {
                Ok(value) => value,
                Err(e) => {
                    log::error!("failed to decode register {}: {}", item.register, e);
                    continue;
                }
            };
            // Process input value
            r.push(Measure {
                name: item.name.clone(),
                help: item.help.clone(),
                labels: item.labels.clone(),
                value,
            });
        }
        // Push result
        Ok(r)
    }
}

fn default_502() -> u16 {
    502
}

fn default_holding() -> RegisterType {
    RegisterType::Holding
}

fn default_5000() -> u64 {
    5_000
}

fn default_255() -> u8 {
    255
}
