// --------------------------------------------------------------------
// Gufo Agent: Agent library
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

pub(crate) mod agent;
pub(crate) mod config;
pub(crate) mod discovery;
pub(crate) mod mdb;
pub(crate) mod registry;
pub(crate) mod resolver;
pub(crate) mod schedule;
pub(crate) mod sender;

pub(crate) use crate::agent::AGENT_DEFAULT_INTERVAL;
pub use crate::agent::{Agent, AgentBuilder, AgentMode};
pub(crate) use config::{AgentConfig, CollectorConfig, Config, SenderConfig};
pub use discovery::config_from_discovery;
pub(crate) use mdb::{MetricsData, MetricsDb};
pub use registry::Collectors;
pub(crate) use resolver::ConfigResolver;
pub(crate) use schedule::Schedule;
pub(crate) use sender::{Sender, SenderCommand};
