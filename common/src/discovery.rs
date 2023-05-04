// --------------------------------------------------------------------
// Gufo Agent: ConfigDiscovery trait
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use crate::AgentError;
use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug)]
pub struct ConfigItem {
    pub config: serde_yaml::Value,
}

impl ConfigItem {
    pub fn from_config<T: Serialize>(value: T) -> Result<Self, AgentError> {
        Ok(Self {
            config: serde_yaml::to_value(value)
                .map_err(|e| AgentError::ConfigurationError(e.to_string()))?,
        })
    }
}

#[derive(Debug, Default)]
pub struct ConfigDiscoveryOpts {
    disabled: HashSet<String>,
}

impl ConfigDiscoveryOpts {
    // Check if the key is disabled
    pub fn is_disabled<T: ToString>(&self, v: T) -> bool {
        self.disabled.contains(&v.to_string())
    }
    #[allow(unused_variables)]
    pub fn get_interval<T: ToString>(name: T) -> u64 {
        10
    }
}

impl TryFrom<String> for ConfigDiscoveryOpts {
    type Error = AgentError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut r = Self::default();
        for opt in value.to_string().split(",") {
            if opt.starts_with("-") {
                r.disabled.insert(opt[1..].to_string());
            }
        }
        Ok(r)
    }
}

#[cfg(test)]
mod tests {
    use super::ConfigDiscoveryOpts;

    #[test]
    fn test_try_from() {
        let opts = ConfigDiscoveryOpts::try_from("foo,-bar,baz,-done".to_string()).unwrap();
        assert!(!opts.is_disabled("foo"));
        assert!(opts.is_disabled("bar"));
        assert!(!opts.is_disabled("baz".to_string()));
        assert!(opts.is_disabled("done".to_string()));
    }
}
