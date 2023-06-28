// --------------------------------------------------------------------
// Gufo Agent: modbus_rtu collector implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use async_trait::async_trait;
use common::{AgentError, Collectable, Measure};
use common::{Labels, LabelsConfig};
use modbus::{ModbusFormat, RegisterType};
use serde::{Deserialize, Serialize};
use tokio::time::{timeout, Duration};
use tokio_modbus::{client::rtu::attach_slave, prelude::Reader, Slave};
use tokio_serial::{DataBits, Parity, SerialPortBuilderExt, StopBits};

const DEFAULT_BAUD_RATE: u32 = 115_200;
const DEFAULT_DATA_BITS: usize = 8;
const DEFAULT_STOP_BITS: usize = 1;

// Collector config
#[derive(Deserialize, Serialize)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    defaults: Option<ConfigDefaults>,
    #[serde(default = "default_5000")]
    pub timeout_ms: u64,
    pub items: Vec<CollectorConfigItem>,
}

#[derive(Deserialize, Serialize)]
pub struct ConfigDefaults {
    serial_path: Option<String>,
    slave: Option<u8>,
    #[serde(default = "default_baud_rate")]
    baud_rate: u32,
    #[serde(default = "default_data_bits")]
    data_bits: usize,
    #[serde(default = "default_parity")]
    parity: CfgParity,
    #[serde(default = "default_stop_bits")]
    stop_bits: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectorConfigItem {
    pub name: String,
    pub help: String,
    pub labels: LabelsConfig,
    pub serial_path: Option<String>,
    pub slave: Option<u8>,
    pub baud_rate: Option<u32>,
    pub data_bits: Option<usize>, // 5,6,7,8
    pub parity: Option<CfgParity>,
    pub stop_bits: Option<usize>, // 1, 2
    pub register: u16,
    #[serde(default = "default_holding")]
    pub register_type: RegisterType,
    pub format: ModbusFormat,
}

#[derive(Deserialize, Serialize, Debug, Clone, Hash)]
#[serde(rename_all = "lowercase")]
pub enum CfgParity {
    None,
    Odd,
    Even,
}

// Collector structure
pub struct Collector {
    timeout_ms: u64,
    items: Vec<CollectorItem>,
}

pub struct CollectorItem {
    pub name: String,
    pub help: String,
    pub labels: Labels,
    serial_path: String,
    slave: Slave,
    baud_rate: u32,
    data_bits: DataBits,
    parity: Parity,
    stop_bits: StopBits,
    register: u16,
    count: u16,
    register_type: RegisterType,
    format: ModbusFormat,
}

// Instantiate collector from given config
impl TryFrom<Config> for Collector {
    type Error = AgentError;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        let mut items = Vec::with_capacity(value.items.len());
        let defaults = value.defaults.unwrap_or_default();
        for c in value.items.iter() {
            let serial_path = match &c.serial_path {
                Some(x) => x.to_string(),
                None => defaults
                    .serial_path
                    .clone()
                    .ok_or_else(|| AgentError::ParseError("serial_path is not set".to_string()))?,
            };
            let slave = Slave::from(match c.slave {
                Some(x) => x,
                None => defaults
                    .slave
                    .ok_or_else(|| AgentError::ParseError("slave is not set".to_string()))?,
            });
            let data_bits = match c.data_bits.unwrap_or(defaults.data_bits) {
                5 => DataBits::Five,
                6 => DataBits::Six,
                7 => DataBits::Seven,
                8 => DataBits::Eight,
                _ => return Err(AgentError::ParseError("invalid data_bits".to_string())),
            };
            let baud_rate = c.baud_rate.unwrap_or(defaults.baud_rate);
            let parity = match c.parity.clone().unwrap_or_else(|| defaults.parity.clone()) {
                CfgParity::None => Parity::None,
                CfgParity::Odd => Parity::Odd,
                CfgParity::Even => Parity::Even,
            };
            let stop_bits = match c.stop_bits.unwrap_or(defaults.stop_bits) {
                1 => StopBits::One,
                2 => StopBits::Two,
                _ => return Err(AgentError::ParseError("invalid stop_bits".to_string())),
            };
            items.push(CollectorItem {
                name: c.name.clone(),
                help: c.help.clone(),
                labels: c.labels.clone().into(),
                serial_path,
                slave,
                baud_rate,
                data_bits,
                parity,
                stop_bits,
                register: c.register,
                count: c.format.min_count(),
                register_type: c.register_type.clone(),
                format: c.format,
            });
        }
        Ok(Self {
            timeout_ms: value.timeout_ms,
            items,
        })
    }
}

// Collector implementation
#[async_trait]
impl Collectable for Collector {
    const NAME: &'static str = "modbus_rtu";
    type Config = Config;

    async fn collect(&mut self) -> Result<Vec<Measure>, AgentError> {
        let duration = Duration::from_millis(self.timeout_ms);
        let mut r = Vec::with_capacity(self.items.len());
        for item in self.items.iter() {
            log::debug!("Setting up serial {}", item.serial_settings());
            let port = tokio_serial::new(item.serial_path.clone(), item.baud_rate)
                .data_bits(item.data_bits)
                .parity(item.parity)
                .stop_bits(item.stop_bits)
                .open_native_async()
                .map_err(|e| AgentError::InternalError(e.to_string()))?;
            // Sending request
            log::debug!("Sending RTU request to slave {}", item.slave);
            let mut ctx = attach_slave(port, item.slave);
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
                timestamp: None,
            });
        }
        Ok(r)
    }
}

impl CollectorItem {
    fn serial_settings(&self) -> String {
        let data_bits = match self.data_bits {
            DataBits::Five => 5,
            DataBits::Six => 6,
            DataBits::Seven => 7,
            DataBits::Eight => 8,
        };
        let parity = match self.parity {
            Parity::None => "N",
            Parity::Even => "E",
            Parity::Odd => "O",
        };
        let stop_bits = match self.stop_bits {
            StopBits::One => 1,
            StopBits::Two => 2,
        };
        format!(
            "{}@{} ({}{}{})",
            self.serial_path, self.baud_rate, data_bits, parity, stop_bits
        )
    }
}

impl Default for ConfigDefaults {
    fn default() -> Self {
        ConfigDefaults {
            serial_path: None,
            slave: None,
            baud_rate: default_baud_rate(),
            data_bits: default_data_bits(),
            parity: default_parity(),
            stop_bits: default_stop_bits(),
        }
    }
}

fn default_holding() -> RegisterType {
    RegisterType::Holding
}

fn default_5000() -> u64 {
    5_000
}

fn default_baud_rate() -> u32 {
    DEFAULT_BAUD_RATE
}

fn default_data_bits() -> usize {
    DEFAULT_DATA_BITS
}

fn default_stop_bits() -> usize {
    DEFAULT_STOP_BITS
}

fn default_parity() -> CfgParity {
    CfgParity::None
}
