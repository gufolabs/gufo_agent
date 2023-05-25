// --------------------------------------------------------------------
// Gufo Agent: Config Resolver
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use crate::Config;
use common::AgentError;
use std::fs;

#[derive(Debug)]
enum ConfigLocation {
    Unknown,
    File(String),
    Url(String),
}

pub struct ConfigResolver {
    location: ConfigLocation,
    //cert_validation: bool,
}

impl ConfigResolver {
    pub fn builder() -> ConfigResolverBuilder {
        ConfigResolverBuilder::default()
    }
    pub fn set_url(&mut self, url: String) -> &mut Self {
        if url.starts_with("http://") || url.starts_with("https://") {
            self.location = ConfigLocation::Url(url);
            return self;
        }
        if let Some(path) = url.strip_prefix("file:") {
            self.location = ConfigLocation::File(path.into());
            return self;
        }
        self.location = ConfigLocation::File(url);
        self
    }
}

#[derive(Default)]
pub struct ConfigResolverBuilder {
    url: Option<String>,
    cert_validation: bool,
}

impl ConfigResolverBuilder {
    pub fn build(&self) -> ConfigResolver {
        let mut resolver = ConfigResolver {
            location: ConfigLocation::Unknown,
            //cert_validation: self.cert_validation,
        };
        if let Some(url) = &self.url {
            resolver.set_url(url.clone());
        }
        resolver
    }
    pub fn set_cert_validation(&mut self, status: bool) -> &mut Self {
        self.cert_validation = status;
        self
    }
    pub fn set_url(&mut self, url: Option<String>) -> &mut Self {
        self.url = url;
        self
    }
}

impl ConfigResolver {
    pub async fn bootstrap(&mut self) -> Result<(), AgentError> {
        log::debug!("Config location is: {:?}", self.location);
        Ok(())
    }
    pub async fn get_config(&self) -> Result<Config, AgentError> {
        // Read config
        let data = match &self.location {
            ConfigLocation::Unknown => {
                return Err(AgentError::BootstrapError(
                    "Config location is not set".into(),
                ))
            }
            ConfigLocation::File(path) => fs::read(path)?,
            ConfigLocation::Url(_) => return Err(AgentError::NotImplementedError),
        };
        // Parse
        let cfg =
            serde_yaml::from_slice(&data).map_err(|e| AgentError::ParseError(e.to_string()))?;
        Ok(cfg)
    }
}
