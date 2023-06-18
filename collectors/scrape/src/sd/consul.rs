// --------------------------------------------------------------------
// Gufo Agent: consul service discovery
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use super::{ServiceDiscovery, LABEL_ADDRESS};
use async_trait::async_trait;
use common::{AgentError, AgentResult, Label};
use relabel::ActiveLabels;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const LABEL_CONSUL_DC: &str = "__meta_consul_dc";
const LABEL_CONSUL_SERVICE: &str = "__meta_consul_service";
const LABEL_CONSUL_SERVICE_ID: &str = "__meta_consul_service_id";
const LABEL_CONSUL_SERVICE_ADDRESS: &str = "__meta_consul_service_address";
const LABEL_CONSUL_SERVICE_PORT: &str = "__meta_consul_service_port";
const LABEL_CONSUL_TAGS: &str = "__meta_consul_tags";

const DEFAULT_TAG_SEPARATOR: &str = ",";

#[derive(Serialize, Deserialize)]
pub(crate) struct ConsulSdConfig {
    #[serde(default = "default_server")]
    server: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<String>,
    #[serde(
        default = "default_tag_separator",
        skip_serializing_if = "is_default_tag_separator"
    )]
    tag_separator: String,
}

pub(crate) struct ConsulSd {
    server: String,
    filter: Option<String>,
    tag_separator: String,
}

impl TryFrom<ConsulSdConfig> for ConsulSd {
    type Error = AgentError;

    fn try_from(value: ConsulSdConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            server: value.server,
            filter: value.filter,
            tag_separator: value.tag_separator,
        })
    }
}

#[async_trait]
impl ServiceDiscovery for ConsulSd {
    async fn get_services(&self) -> AgentResult<Vec<ActiveLabels>> {
        let mut query = Vec::default();
        if let Some(filter) = &self.filter {
            query.push(format!("filter={}", filter))
        }
        let q = if query.is_empty() {
            "".into()
        } else {
            format!("?{}", query.join("&"))
        };
        let url = format!("http://{}/v1/agent/services{}", self.server, q);
        log::debug!("Requesting {}", url);
        let client = reqwest::Client::builder()
            .gzip(true)
            .build()
            .map_err(|e| AgentError::InternalError(e.to_string()))?;
        let resp = client
            .get(&url)
            .send()
            .await
            .map_err(|e| AgentError::InternalError(e.to_string()))?;
        let out: HashMap<String, AgentService> = resp
            .json()
            .await
            .map_err(|e| AgentError::ParseError(e.to_string()))?;
        let mut r = Vec::with_capacity(out.len());
        for svc in out.values() {
            let mut items = Vec::with_capacity(7 + svc.meta.len());
            // __address__
            items.push(Label::new(
                LABEL_ADDRESS,
                format!("{}:{}", svc.address, svc.port),
            ));
            // __meta_consul_dc
            items.push(Label::new(LABEL_CONSUL_DC, svc.datacenter.to_owned()));
            // __meta_consul_service
            items.push(Label::new(LABEL_CONSUL_SERVICE, svc.service.to_owned()));
            // __meta_consul_service_id
            items.push(Label::new(LABEL_CONSUL_SERVICE_ID, svc.id.to_owned()));
            // __meta_consul_service_address
            items.push(Label::new(
                LABEL_CONSUL_SERVICE_ADDRESS,
                svc.address.to_owned(),
            ));
            // __meta_consul_service_port
            items.push(Label::new(LABEL_CONSUL_SERVICE_PORT, svc.port.to_string()));
            // __meta_consul_tags
            items.push(Label::new(
                LABEL_CONSUL_TAGS,
                svc.tags.join(&self.tag_separator),
            ));
            // __meta_consul_meta_XXX
            for (k, v) in svc.meta.iter() {
                items.push(Label::new(
                    format!("__meta_consul_meta_{}", k),
                    v.to_owned(),
                ));
            }
            // Push labels
            r.push(ActiveLabels::new(items));
        }
        Ok(r)
    }
}

#[derive(Deserialize, Debug)]
struct AgentService {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "Service")]
    service: String,
    #[serde(rename = "Address")]
    address: String,
    #[serde(rename = "Port")]
    port: u16,
    #[serde(rename = "Datacenter")]
    datacenter: String,
    #[serde(rename = "Tags")]
    tags: Vec<String>,
    #[serde(rename = "Meta")]
    meta: HashMap<String, String>,
}

fn default_server() -> String {
    "127.0.0.1:8500".into()
}

fn default_tag_separator() -> String {
    DEFAULT_TAG_SEPARATOR.into()
}

fn is_default_tag_separator(v: &String) -> bool {
    v == DEFAULT_TAG_SEPARATOR
}
