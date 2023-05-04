// --------------------------------------------------------------------
// Gufo Agent: Collectatble trait
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------
use crate::{AgentError, ConfigDiscoveryOpts, ConfigItem, Labels};
use async_trait::async_trait;

#[derive(Debug, Copy, Clone)]
pub enum Value {
    Counter(u64),
    Gauge(u64),
    GaugeI(i64),
    GaugeF(f32),
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Counter(x) => x.to_string(),
            Value::Gauge(x) => x.to_string(),
            Value::GaugeI(x) => x.to_string(),
            Value::GaugeF(x) => x.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Measure {
    pub name: &'static str,
    pub help: &'static str,
    pub value: Value,
    pub labels: Labels,
}

#[async_trait]
pub trait Collectable
where
    Self: Sized + TryFrom<Self::Config>,
{
    const NAME: &'static str;
    const RANDOM_OFFSET: bool = true;
    type Config;

    fn get_name() -> &'static str {
        Self::NAME
    }
    fn is_random_offset() -> bool {
        Self::RANDOM_OFFSET
    }
    async fn collect(&mut self) -> Result<Vec<Measure>, AgentError>;
    #[allow(unused_variables)]
    fn discover_config(opts: &ConfigDiscoveryOpts) -> Result<Vec<ConfigItem>, AgentError> {
        Ok(Vec::new())
    }
}
