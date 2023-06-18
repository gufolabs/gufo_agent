// --------------------------------------------------------------------
// Gufo Agent: dns service discovery
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use super::{ServiceDiscovery, LABEL_ADDRESS};
use async_trait::async_trait;
use common::{AgentError, AgentResult, Label};
use relabel::ActiveLabels;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use trust_dns_proto::rr::{record_type::RecordType, RData};
use trust_dns_resolver::TokioAsyncResolver;

const DEFAULT_QUERY_TYPE: &str = "A";

#[derive(Serialize, Deserialize)]
pub(crate) struct DnsSdConfig {
    query: String,
    #[serde(default = "default_a", skip_serializing_if = "is_a")]
    query_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<u16>,
}

pub(crate) struct DnsSd {
    query: String,
    query_type: RecordType,
    port: u16,
}

impl TryFrom<DnsSdConfig> for DnsSd {
    type Error = AgentError;

    fn try_from(value: DnsSdConfig) -> Result<Self, Self::Error> {
        if value.query_type != "A" && value.query_type != "SRV" {
            return Err(AgentError::ConfigurationError(
                "query_type must be A or SRV".into(),
            ));
        }
        match value.port {
            Some(_) => {
                if value.query_type != "A" {
                    return Err(AgentError::ConfigurationError(
                        "port should be set only for query_type A".into(),
                    ));
                }
            }
            None => {
                if value.query_type == "A" {
                    return Err(AgentError::ConfigurationError(
                        "port must be set for query_type A".into(),
                    ));
                }
            }
        }
        Ok(Self {
            query: value.query.clone(),
            query_type: RecordType::from_str(&value.query_type)
                .map_err(|e| AgentError::ConfigurationError(e.to_string()))?,
            port: value.port.unwrap_or_default(),
        })
    }
}

#[async_trait]
impl ServiceDiscovery for DnsSd {
    async fn get_services(&self) -> AgentResult<Vec<ActiveLabels>> {
        let resolver = TokioAsyncResolver::tokio_from_system_conf()
            .map_err(|e| AgentError::ConfigurationError(e.to_string()))?;
        let lookup = resolver
            .lookup(self.query.clone(), self.query_type)
            .await
            .map_err(|e| AgentError::InternalError(e.to_string()))?;
        let mut r = Vec::new();
        for record in lookup.record_iter() {
            match record.data() {
                Some(RData::A(addr)) => r.push(ActiveLabels::new(vec![Label::new(
                    LABEL_ADDRESS,
                    format!("{}:{}", addr, self.port),
                )])),
                Some(RData::SRV(srv)) => {
                    // Resolve target to a
                    let srv_lookup = resolver
                        .lookup(srv.target().to_string(), RecordType::A)
                        .await
                        .map_err(|e| AgentError::InternalError(e.to_string()))?;
                    for tr in srv_lookup.record_iter() {
                        match tr.data() {
                            Some(RData::A(addr)) => r.push(ActiveLabels::new(vec![Label::new(
                                LABEL_ADDRESS,
                                format!("{}:{}", addr, srv.port()),
                            )])),
                            _ => continue,
                        }
                    }
                }
                _ => continue,
            };
        }
        Ok(r)
    }
}

fn default_a() -> String {
    DEFAULT_QUERY_TYPE.into()
}
fn is_a(v: &String) -> bool {
    v == DEFAULT_QUERY_TYPE
}
