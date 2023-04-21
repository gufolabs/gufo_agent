// --------------------------------------------------------------------
// Gufo Agent: Collectatble trait
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------
use crate::{AgentError, Labels};
use async_trait::async_trait;

#[derive(Debug, Copy, Clone)]
pub enum Value {
    Counter(u64),
    Gauge(u64),
    GaugeI(i64),
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Counter(x) => format!("{}", x),
            Value::Gauge(x) => format!("{}", x),
            Value::GaugeI(x) => format!("{}", x),
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
    type Config;

    fn get_name() -> &'static str {
        Self::NAME
    }
    async fn collect(&mut self) -> Result<Vec<Measure>, AgentError>;
}
