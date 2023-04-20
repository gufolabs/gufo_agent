// --------------------------------------------------------------------
// Gufo Agent: Agent library
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

pub(crate) mod agent;
pub(crate) mod config;
pub(crate) mod registry;
pub(crate) mod resolver;
pub(crate) mod schedule;
pub(crate) mod sender;

pub use agent::{Agent, AgentBuilder};
pub(crate) use config::{CollectorConfig, Config};
pub use registry::Collectors;
pub(crate) use resolver::ConfigResolver;
pub(crate) use schedule::Schedule;
pub(crate) use sender::{MetricsData, Sender, SenderCommand};
