// --------------------------------------------------------------------
// Gufo Agent: mysql_query collector implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use async_trait::async_trait;
use bigdecimal::{BigDecimal, ToPrimitive};
use common::{AgentError, AgentResult, Collectable, Label, Labels, LabelsConfig, Measure, Value};
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::{
    mysql::{MySqlConnectOptions, MySqlConnection, MySqlRow},
    Column, Connection, Row, TypeInfo,
};

// Collector config
#[derive(Deserialize, Serialize)]
pub struct Config {
    host: Option<String>,
    port: Option<u16>,
    username: Option<String>,
    password: Option<String>,
    database: Option<String>,
    items: Vec<ConfigQueryItem>,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigQueryItem {
    query: String,
    // name
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_column: Option<String>,
    // help
    #[serde(skip_serializing_if = "Option::is_none")]
    help: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    help_column: Option<String>,
    // labels
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: LabelsConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    label_columns: Option<Vec<String>>,
    // value
    #[serde(default = "default_value")]
    value_column: String,
}

// Collector structure
pub struct Collector {
    connect_opts: MySqlConnectOptions,
    items: Vec<QueryItem>,
}

pub struct QueryItem {
    query: String,
    name: ColumnReference,
    help: ColumnReference,
    labels: Labels,
    label_columns: Option<Vec<String>>,
    value_column: String,
}

pub enum ColumnReference {
    Fixed(String),
    Column(String),
}

impl TryFrom<(Option<&String>, Option<&String>)> for ColumnReference {
    type Error = AgentError;

    // fixed/column
    fn try_from((fixed, column): (Option<&String>, Option<&String>)) -> Result<Self, Self::Error> {
        if let Some(c) = &column {
            return Ok(ColumnReference::Column(c.to_string()));
        }
        if let Some(f) = fixed {
            return Ok(ColumnReference::Fixed(f.clone()));
        }
        Err(AgentError::ConfigurationError(
            "fixed or column value must be set".to_string(),
        ))
    }
}

// Instantiate collector from given config
impl TryFrom<Config> for Collector {
    type Error = AgentError;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        let mut connect_opts = MySqlConnectOptions::new();
        if let Some(host) = &value.host {
            connect_opts = connect_opts.host(host.as_str());
        }
        if let Some(port) = &value.port {
            connect_opts = connect_opts.port(*port);
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
        // Parse items
        let mut items = Vec::with_capacity(value.items.len());
        for ci in value.items.iter() {
            // name
            let name = ColumnReference::try_from((ci.name.as_ref(), ci.name_column.as_ref()))
                .map_err(|_| {
                    AgentError::ConfigurationError(
                        "either name or name_column must be set".to_string(),
                    )
                })?;
            // help
            let help = ColumnReference::try_from((ci.help.as_ref(), ci.help_column.as_ref()))
                .unwrap_or(ColumnReference::Fixed("".to_string()));
            items.push(QueryItem {
                query: ci.query.clone(),
                name,
                help,
                labels: ci.labels.clone().into(),
                label_columns: ci.label_columns.clone(),
                value_column: ci.value_column.clone(),
            });
        }
        Ok(Self {
            connect_opts,
            items,
        })
    }
}

#[derive(Debug)]
struct RowFields {
    name: RowColumnReference,
    help: RowColumnReference,
    value: usize,
    target: TargetType,
    labels: Labels,
    label_columns: Option<Vec<LabelColumn>>,
}

#[derive(Debug)]
enum RowColumnReference {
    Fixed(String),
    Ordinal(usize),
}

#[derive(Debug)]
struct LabelColumn {
    name: String,
    ordinal: usize,
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

impl TryFrom<(&QueryItem, &MySqlRow)> for RowFields {
    type Error = AgentError;

    fn try_from((qi, row): (&QueryItem, &MySqlRow)) -> Result<Self, Self::Error> {
        let name = match &qi.name {
            ColumnReference::Fixed(x) => RowColumnReference::Fixed(x.clone()),
            ColumnReference::Column(x) => RowColumnReference::Ordinal(
                row.try_column(x.as_str())
                    .map_err(|_| AgentError::InternalError(format!("'{}' is not found", x)))?
                    .ordinal(),
            ),
        };
        let help: RowColumnReference = match &qi.help {
            ColumnReference::Fixed(x) => RowColumnReference::Fixed(x.clone()),
            ColumnReference::Column(x) => RowColumnReference::Ordinal(
                row.try_column(x.as_str())
                    .map_err(|_| AgentError::InternalError(format!("'{}' is not found", x)))?
                    .ordinal(),
            ),
        };
        // Check name column type
        // Value column
        let value_column = row.try_column(qi.value_column.as_str()).map_err(|_| {
            AgentError::InternalError(format!("'{}' is not found", qi.value_column))
        })?;
        // Check value column type
        // NB: PgType is unreachable and all relevant methods are hidden
        //     So using dumb implementation with strings.
        let target = match value_column.type_info().name() {
            "SMALLINT" => TargetType::I16,
            "INT" => TargetType::I32,
            "BIGINT" => TargetType::I64,
            "FLOAT" => TargetType::F32,
            "DOUBLE" => TargetType::F64,
            "DECIMAL" => TargetType::Numeric,
            _ => {
                return Err(AgentError::InternalError(format!(
                    "unsupported column type {}",
                    value_column.type_info().name()
                )))
            }
        };
        // Process label_columns if any
        let label_columns = match &qi.label_columns {
            Some(cols) => {
                let mut r = Vec::with_capacity(cols.len());
                for cn in cols.iter() {
                    let lc = row
                        .try_column(cn.as_str())
                        .map_err(|_| AgentError::InternalError(format!("'{}' is not found", cn)))?;
                    r.push(LabelColumn {
                        name: cn.to_string(),
                        ordinal: lc.ordinal(),
                    });
                }
                Some(r)
            }
            None => None,
        };
        Ok(RowFields {
            name,
            value: value_column.ordinal(),
            target,
            help,
            labels: qi.labels.clone(),
            label_columns,
        })
    }
}

impl RowFields {
    fn measure(&self, row: &MySqlRow) -> AgentResult<Measure> {
        let name = match &self.name {
            RowColumnReference::Fixed(x) => x.clone(),
            RowColumnReference::Ordinal(x) => row.get(x),
        };
        let help = match &self.help {
            RowColumnReference::Fixed(x) => x.clone(),
            RowColumnReference::Ordinal(x) => row.get(x),
        };
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
        //
        let column_labels = match &self.label_columns {
            Some(cols) => {
                let mut r = Vec::with_capacity(cols.len());
                for cl in cols.iter() {
                    // @todo: Allow NULL fields
                    r.push(Label::new(
                        cl.name.clone(),
                        row.get::<String, usize>(cl.ordinal),
                    ));
                }
                Labels::new(r)
            }
            None => Labels::default(),
        };
        Ok(Measure {
            name,
            help,
            labels: Labels::merge_sort2(&self.labels, &column_labels),
            value,
        })
    }
}

// Collector implementation
#[async_trait]
impl Collectable for Collector {
    const NAME: &'static str = "mysql_query";
    type Config = Config;

    async fn collect(&mut self) -> Result<Vec<Measure>, AgentError> {
        // Connect to database
        let mut conn = MySqlConnection::connect_with(&self.connect_opts)
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

fn default_value() -> String {
    "value".into()
}
