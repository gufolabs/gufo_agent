// --------------------------------------------------------------------
// Gufo Agent: Openmetrics sender implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use common::{Labels, Measure, Value};
use std::collections::{BTreeMap, HashMap};
use tokio::sync::mpsc;

pub(crate) enum SenderCommand {
    Data(MetricsData),
    SetAgentLabels(Labels), // @todo: Rename to SetAgentLabels
}

#[derive(Debug)]
pub(crate) struct MetricsData {
    pub collector: &'static str,
    // collector labels
    pub labels: Labels,
    // collector measures
    pub measures: Vec<Measure>,
    // Timestamp in UNIX format
    pub ts: u64,
}

pub(crate) struct Sender {
    rx: mpsc::Receiver<SenderCommand>,
    tx: mpsc::Sender<SenderCommand>,
    metrics: BTreeMap<MetricFamilyKey, MetricFamilyData>,
    agent_labels: Labels,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone)]
struct MetricFamilyKey {
    collector: &'static str,
    name: &'static str,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct OutputItem {
    labels: Labels,
    value: String,
    ts: u64,
}

#[derive(Debug)]
enum ValueType {
    Counter,
    Gauge,
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
            Value::Gauge(_) => ValueType::Gauge,
        }
    }
}

#[derive(Debug)]
struct MetricFamilyData {
    help: &'static str,
    r#type: ValueType,
    values: HashMap<Labels, MetricValue>,
}

#[derive(Debug)]
struct MetricValue {
    value: Value,
    collector_labels: Labels,
    ts: u64,
}

const SENDER_CHANNEL_BUFFER: usize = 10_000;

impl Default for Sender {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel::<SenderCommand>(SENDER_CHANNEL_BUFFER);
        Self {
            rx,
            tx,
            metrics: BTreeMap::new(),
            agent_labels: Labels::empty(),
        }
    }
}

impl Sender {
    // Get cloned tx channel
    pub fn get_tx(&self) -> mpsc::Sender<SenderCommand> {
        self.tx.clone()
    }
    // Run sender message processing
    pub async fn run(&mut self) {
        log::info!("Running sender");
        while let Some(msg) = self.rx.recv().await {
            match msg {
                SenderCommand::Data(data) => self.apply_data(&data),
                SenderCommand::SetAgentLabels(labels) => {
                    log::debug!("Set labels to: {:?}", labels);
                    self.agent_labels = labels
                } //SenderCommand::Shutdown => break,
            }
        }
        log::info!("Shutting down");
    }
    //
    fn apply_data(&mut self, data: &MetricsData) {
        for measure in data.measures.iter() {
            // Check for Metric Family
            let k = MetricFamilyKey {
                collector: data.collector,
                name: measure.name,
            };
            // @todo: Use .get()
            if !self.metrics.contains_key(&k) {
                // Insert metric family info
                self.metrics.insert(
                    k.clone(),
                    MetricFamilyData {
                        help: measure.help,
                        r#type: measure.value.into(),
                        values: HashMap::new(),
                    },
                );
            }
            //
            if let Some(family) = self.metrics.get_mut(&k) {
                family.values.insert(
                    measure.labels.clone(),
                    MetricValue {
                        value: measure.value,
                        collector_labels: data.labels.clone(),
                        ts: data.ts,
                    },
                );
            }
        }
        self.dump();
    }
    fn dump(&self) {
        for (family, fv) in self.metrics.iter() {
            println!("# HELP {}_{} {}", family.collector, family.name, fv.help);
            println!(
                "# TYPE {}_{} {}",
                family.collector,
                family.name,
                fv.r#type.as_str()
            );
            let mut items: Vec<OutputItem> = fv
                .values
                .iter()
                .map(|(labels, value)| OutputItem {
                    labels: Labels::merge_sort3(
                        &self.agent_labels,
                        &value.collector_labels,
                        labels,
                    ),
                    value: value.value.to_string(),
                    ts: value.ts,
                })
                .collect();
            items.sort();
            for item in items.iter() {
                println!(
                    "{}_{}{} {}{}",
                    family.collector,
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
                    }
                )
            }
        }
    }
}
