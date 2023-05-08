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
    disable_builtins: bool,
    disabled: HashSet<String>,
    explicitly_enabled: HashSet<String>,
}

impl ConfigDiscoveryOpts {
    // Check if the key is disabled
    pub fn is_disabled<T: ToString>(&self, v: T) -> bool {
        if self.disable_builtins {
            !self.explicitly_enabled.contains(&v.to_string())
        } else {
            self.disabled.contains(&v.to_string())
        }
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
        for opt in value.split(',') {
            if opt == "-builtins" {
                r.disable_builtins = true;
                continue;
            }
            if let Some(stripped) = opt.strip_prefix('-') {
                r.disabled.insert(stripped.to_string());
                continue;
            }
            if let Some(stripped) = opt.strip_prefix('+') {
                r.explicitly_enabled.insert(stripped.to_string());
                continue;
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

    #[test]
    fn test_try_disable_builtins() {
        let opts = ConfigDiscoveryOpts::try_from("foo,-bar,-builtins".to_string()).unwrap();
        assert!(opts.is_disabled("foo"));
        assert!(opts.is_disabled("bar"));
        assert!(opts.is_disabled("baz".to_string()));
        assert!(opts.is_disabled("done".to_string()));
    }
    #[test]
    fn test_try_explicit_enable() {
        let opts = ConfigDiscoveryOpts::try_from("foo,+bar,-builtins".to_string()).unwrap();
        assert!(opts.is_disabled("foo"));
        assert!(!opts.is_disabled("bar"));
        assert!(opts.is_disabled("baz".to_string()));
        assert!(opts.is_disabled("done".to_string()));
    }
}
