// --------------------------------------------------------------------
// Gufo Agent: sql collector implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use async_trait::async_trait;
use bigdecimal::{BigDecimal, ToPrimitive};
use common::{AgentError, AgentResult, Collectable, Labels, Measure, Value};
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgConnectOptions, PgConnection, PgRow},
    Column, Connection, Row, TypeInfo,
};

// Collector config
#[derive(Deserialize, Serialize)]
pub struct Config {
    host: Option<String>,
    port: Option<u16>,
    socket: Option<String>,
    username: Option<String>,
    password: Option<String>,
    database: Option<String>,
    items: Vec<ConfigQueryItem>,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigQueryItem {
    query: String,
    #[serde(default = "default_name")]
    name_column: String,
    #[serde(default = "default_value")]
    value_column: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    help_column: Option<String>,
}

// Collector structure
pub struct Collector {
    connect_opts: PgConnectOptions,
    items: Vec<QueryItem>,
}

pub struct QueryItem {
    query: String,
    name_column: String,
    value_column: String,
    help_column: Option<String>,
}

// Instantiate collector from given config
impl TryFrom<Config> for Collector {
    type Error = AgentError;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        let mut connect_opts = PgConnectOptions::new().application_name("gufo-agent");
        if let Some(host) = &value.host {
            connect_opts = connect_opts.host(host.as_str());
        }
        if let Some(port) = &value.port {
            connect_opts = connect_opts.port(*port);
        }
        if let Some(socket) = &value.socket {
            connect_opts = connect_opts.socket(socket.as_str());
        }
        if let Some(username) = &value.username {
            connect_opts = connect_opts.username(username.as_str());
        }
        if let Some(password) = &value.password {
            connect_opts = connect_opts.password(password.as_str());
        }
        if let Some(database) = &value.database {
            connect_opts = connect_opts.database(database);
        }
        Ok(Self {
            connect_opts,
            items: value
                .items
                .iter()
                .map(|x| QueryItem {
                    query: x.query.clone(),
                    name_column: x.name_column.clone(),
                    value_column: x.value_column.clone(),
                    help_column: x.help_column.clone(),
                })
                .collect(),
        })
    }
}

#[derive(Debug)]
struct RowFields {
    name: usize,
    value: usize,
    target: TargetType,
    labels: Option<usize>,
    help: Option<usize>,
}

#[derive(Debug)]
enum TargetType {
    I16,
    I32,
    I64,
    F32,
    F64,
    Numeric,
}

impl TryFrom<(&QueryItem, &PgRow)> for RowFields {
    type Error = AgentError;

    fn try_from(value: (&QueryItem, &PgRow)) -> Result<Self, Self::Error> {
        let (qi, row) = value;
        let name_column = row
            .try_column(qi.name_column.as_str())
            .map_err(|_| AgentError::InternalError(format!("'{}' is not found", qi.name_column)))?;
        // Check name column type
        // Value column
        let value_column = row
            .try_column(qi.value_column.as_str())
            .map_err(|_| AgentError::InternalError(format!("'{}' is not found", qi.name_column)))?;
        // Check value column type
        // NB: PgType is unreachable and all relevant methods are hidden
        //     So using dumb implementation with strings.
        let target = match value_column.type_info().name() {
            "SMALLINT" | "SMALLSERIAL" | "INT2" => TargetType::I16,
            "INT" | "INT4" | "SERIAL" => TargetType::I32,
            "BIGINT" | "BIGSERIAL" | "INT8" => TargetType::I64,
            "REAL" | "FLOAT4" => TargetType::F32,
            "DOUBLE PRECISION" | "FLOAT8" => TargetType::F64,
            "NUMERIC" => TargetType::Numeric,
            _ => {
                return Err(AgentError::InternalError(format!(
                    "unsupported column type {}",
                    value_column.type_info().name()
                )))
            }
        };
        // help
        let help = match &qi.help_column {
            Some(h) => Some(
                row.try_column(h.as_str())
                    .map_err(|_| AgentError::InternalError(format!("'{}' is not found", h)))?
                    .ordinal(),
            ),
            None => None,
        };
        Ok(RowFields {
            name: name_column.ordinal(),
            value: value_column.ordinal(),
            target,
            labels: None,
            help,
        })
    }
}

impl RowFields {
    fn measure(&self, row: &PgRow) -> AgentResult<Measure> {
        let name = row.get(self.name);
        let value = match self.target {
            TargetType::I16 => Value::GaugeI(row.get::<i16, usize>(self.value) as i64),
            TargetType::I32 => Value::GaugeI(row.get::<i32, usize>(self.value) as i64),
            TargetType::I64 => Value::GaugeI(row.get::<i64, usize>(self.value)),
            TargetType::F32 => Value::GaugeF(row.get::<f32, usize>(self.value)),
            TargetType::F64 => Value::GaugeF(row.get::<f64, usize>(self.value) as f32),
            TargetType::Numeric => Value::GaugeF(
                row.get::<BigDecimal, usize>(self.value)
                    .to_f32()
                    .ok_or(AgentError::ParseError("Failed to decode value".to_string()))?,
            ),
        };
        let help = match self.help {
            Some(h) => row.get(h),
            None => "".into(),
        };
        Ok(Measure {
            name,
            help,
            labels: Labels::default(),
            value,
        })
    }
}

// Collector implementation
#[async_trait]
impl Collectable for Collector {
    const NAME: &'static str = "postgres_query";
    type Config = Config;

    async fn collect(&mut self) -> Result<Vec<Measure>, AgentError> {
        // Connect to database
        let mut conn = PgConnection::connect_with(&self.connect_opts)
            .await
            .map_err(|e| AgentError::InternalError(e.to_string()))?;
        // Collect data
        let mut r = Vec::new();
        for item in self.items.iter() {
            let mut fields: Option<RowFields> = None;
            let mut rows = sqlx::query(&item.query).persistent(false).fetch(&mut conn);
            while let Some(row) = rows
                .try_next()
                .await
                .map_err(|e| AgentError::InternalError(e.to_string()))?
            {
                if fields.is_none() {
                    fields = match RowFields::try_from((item, &row)) {
                        Ok(f) => Some(f),
                        Err(e) => {
                            log::error!("Invalid query. Skipping: {}", e);
                            break;
                        }
                    }
                }
                if let Some(f) = &fields {
                    match f.measure(&row) {
                        Ok(m) => r.push(m),
                        Err(e) => {
                            log::error!("Cannot decode row: {}", e);
                            continue;
                        }
                    }
                }
            }
        }
        Ok(r)
    }
    // !!! Uncomment for config discovery
    // fn discover_config(_: &ConfigDiscoveryOpts) -> Result<Vec<ConfigItem>, AgentError> {
    //     let cfg = Config;
    //     Ok(vec![ConfigItem::from_config(cfg)?])
    // }
}

fn default_name() -> String {
    "name".into()
}

fn default_value() -> String {
    "value".into()
}
