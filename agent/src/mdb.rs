// --------------------------------------------------------------------
// Gufo Agent: Metrics database
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use crate::{ActiveLabels, RelabelRuleset};
use bytes::BytesMut;
use common::{AgentError, Labels, Measure, Value};
use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::RwLock;

struct _Inner {
    data: BTreeMap<MetricFamilyKey, MetricFamilyData>,
    labels: Labels,
}

pub(crate) struct MetricsDb(Arc<RwLock<_Inner>>);

#[derive(Debug)]
pub(crate) struct MetricsData {
    pub collector: &'static str,
    // collector labels
    pub labels: Arc<Labels>,
    // Relabeling ruleset
    pub relabel: Arc<Option<RelabelRuleset>>,
    // collector measures
    pub measures: Vec<Measure>,
    // Timestamp in UNIX format
    pub ts: u64,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone)]
struct MetricFamilyKey {
    collector: &'static str,
    name: String,
}

#[derive(Debug)]
struct MetricFamilyData {
    help: String,
    r#type: ValueType,
    values: HashMap<Labels, MetricValue>,
}

#[derive(Debug)]
struct MetricValue {
    value: Value,
    ts: u64,
}

#[derive(Debug)]
enum ValueType {
    Counter,
    Gauge,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct OutputItem<'a> {
    labels: &'a Labels,
    value: String,
    ts: u64,
}

impl ValueType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ValueType::Counter => "counter",
            ValueType::Gauge => "gauge",
        }
    }
}

impl From<Value> for ValueType {
    fn from(value: Value) -> Self {
        match value {
            Value::Counter(_) => ValueType::Counter,
            Value::CounterF(_) => ValueType::Counter,
            Value::Gauge(_) => ValueType::Gauge,
            Value::GaugeI(_) => ValueType::Gauge,
            Value::GaugeF(_) => ValueType::Gauge,
        }
    }
}

impl Default for MetricsDb {
    fn default() -> Self {
        Self(Arc::new(RwLock::new(_Inner {
            data: BTreeMap::new(),
            labels: Labels::default(),
        })))
    }
}

impl MetricsDb {
    pub async fn set_labels(&mut self, labels: Labels) {
        let mut db = self.0.write().await;
        db.labels = labels
    }
    pub async fn apply_data(&mut self, data: &MetricsData) {
        let mut db = self.0.write().await;
        for measure in data.measures.iter() {
            // Relabeling
            if let Some(measure) = match Option::as_ref(&data.relabel) {
                Some(ruleset) => match ruleset.process(&db.labels, &data.labels, measure) {
                    Ok(r) => r,
                    Err(e) => {
                        log::error!("Failed to relabel: {}", e);
                        continue;
                    }
                },
                None => {
                    match ActiveLabels::try_from((&db.labels, data.labels.deref(), &measure.labels))
                    {
                        Ok(labels) => Some(labels.to_measure(measure)),
                        Err(e) => {
                            log::error!("Failed to expand labels: {}", e);
                            continue;
                        }
                    }
                }
            } {
                // Check for Metric Family
                let k = MetricFamilyKey {
                    collector: data.collector,
                    name: measure.name.clone(),
                };
                // @todo: Use .get()
                // Wait for map_try_insert feature to stabilize
                if !db.data.contains_key(&k) {
                    // Insert metric family info
                    db.data.insert(
                        k.clone(),
                        MetricFamilyData {
                            help: measure.help.clone(),
                            r#type: measure.value.into(),
                            values: HashMap::new(),
                        },
                    );
                }
                if let Some(family) = db.data.get_mut(&k) {
                    // Install value if not dropped
                    family.values.insert(
                        measure.labels,
                        MetricValue {
                            value: measure.value,
                            ts: measure.timestamp.unwrap_or(data.ts),
                        },
                    );
                }
            }
        }
    }
    pub async fn write_openmetrics(&self, out: &mut BytesMut) -> Result<(), AgentError> {
        let db = self.0.read().await;
        for (family, fv) in db.data.iter() {
            if !fv.help.is_empty() {
                fmt::write(out, format_args!("# HELP {} {}\n", family.name, fv.help,))?;
            }
            fmt::write(
                out,
                format_args!("# TYPE {} {}\n", family.name, fv.r#type.as_str(),),
            )?;
            let mut items: Vec<OutputItem> = fv
                .values
                .iter()
                .map(|(labels, value)| OutputItem {
                    labels,
                    value: value.value.to_string(),
                    ts: value.ts,
                })
                .collect();
            items.sort();
            for item in items.iter() {
                fmt::write(
                    out,
                    format_args!(
                        "{}{} {}{}\n",
                        family.name,
                        if item.labels.is_empty() {
                            "".into()
                        } else {
                            format!("{{{}}}", item.labels.to_openmetrics())
                        },
                        item.value,
                        if item.ts > 0 {
                            format!(" {}", item.ts)
                        } else {
                            "".to_string()
                        },
                    ),
                )?;
            }
        }
        fmt::write(out, format_args!("# EOF\n"))?;
        Ok(())
    }
    pub async fn to_openmetrics_string(&self) -> Result<String, AgentError> {
        let mut buf = BytesMut::with_capacity(16 * 1024);
        self.write_openmetrics(&mut buf).await?;
        String::from_utf8(buf[..].to_vec()).map_err(|e| AgentError::InternalError(e.to_string()))
    }
}

impl Clone for MetricsDb {
    fn clone(&self) -> Self {
        MetricsDb(Arc::clone(&self.0))
    }
}
