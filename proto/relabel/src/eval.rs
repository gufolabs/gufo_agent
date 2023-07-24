// --------------------------------------------------------------------
// Gufo Agent: Label evaluation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use super::{ActiveLabels, RelabelRuleConfig};
use aho_corasick::AhoCorasick;
use common::{AgentError, AgentResult};
use regex::Regex;
use std::sync::OnceLock;

#[derive(Debug)]
pub(crate) struct Eval {
    source_labels: Vec<String>,
    separator: String,
    regex: Option<Regex>,
    replacement: Option<String>,
}

static CAP_RX: OnceLock<Regex> = OnceLock::new();
static AC_PATTERNS: &[&str; 4] = &["$0", "${0}", "$1", "${1}"];

impl TryFrom<&RelabelRuleConfig> for Eval {
    type Error = AgentError;

    fn try_from(value: &RelabelRuleConfig) -> Result<Self, Self::Error> {
        let source_labels = value.source_labels.clone().unwrap_or_default();
        // Compile regex
        let regex = match &value.regex {
            Some(rx) => {
                Some(Regex::new(rx).map_err(|e| AgentError::ConfigurationError(e.to_string()))?)
            }
            None => None,
        };
        // Rewrite $1 -> ${1}
        let replacement = value.replacement.clone().map(|x| {
            let cap_rx = CAP_RX.get_or_init(|| Regex::new("(^|[^$])\\$(\\d+)").unwrap());
            cap_rx.replace_all(&x, "${1}$${${2}}").to_string()
        });
        //
        Ok(Eval {
            source_labels,
            separator: value.separator.clone(),
            regex,
            replacement,
        })
    }
}

impl Eval {
    pub(crate) fn require_source_labels(&self) -> AgentResult<()> {
        if self.source_labels.is_empty() {
            return Err(AgentError::ConfigurationError(
                "'source_labels' must be set".to_string(),
            ));
        }
        Ok(())
    }
    // Resulting string, if match. None otherwise
    pub(crate) fn apply(&self, labels: &ActiveLabels) -> Option<String> {
        if self.source_labels.is_empty() {
            // Add
            return self.replacement.clone();
        }
        let mut values = Vec::with_capacity(self.source_labels.len());
        for n in self.source_labels.iter() {
            match labels.get(n) {
                Some(v) => values.push(v.clone()),
                None => return None,
            }
        }
        let mut r = values.join(self.separator.as_str());
        // Apply regex
        match &self.regex {
            Some(rx) => {
                match rx.captures(r.as_str()) {
                    Some(caps) => {
                        if let Some(repl) = &self.replacement {
                            // Apply replacement
                            let mut x = String::new();
                            caps.expand(repl, &mut x);
                            r = x;
                        }
                    }
                    None => return None, // Not matched
                }
            }
            None => {
                // $0, $1
                if let Some(repl) = &self.replacement {
                    let ac = AhoCorasick::new(AC_PATTERNS).unwrap();
                    let mut x = String::new();
                    ac.replace_all_with(repl, &mut x, |_, _, dst| {
                        dst.push_str(&r);
                        true
                    });
                    r = x;
                }
            }
        }
        Some(r)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{ActiveLabels, Eval};
    use common::Label;
    use regex::Regex;

    fn get_labels() -> ActiveLabels {
        let mut labels = ActiveLabels::default();
        labels.insert(Label::new("zzz", "xxx"));
        labels.insert(Label::new("subsystem", "kata"));
        labels.insert(Label::new("server", "web000"));
        labels
    }

    #[test]
    fn test_empty_rule() {
        let labels = get_labels();
        let rule = Eval {
            source_labels: vec![],
            separator: ";".into(),
            regex: None,
            replacement: None,
        };
        assert_eq!(rule.apply(&labels), None);
    }

    #[test]
    fn test_concat() {
        let labels = get_labels();
        let rule = Eval {
            source_labels: vec!["server".to_string(), "subsystem".to_string()],
            separator: "@".into(),
            regex: None,
            replacement: None,
        };
        assert_eq!(rule.apply(&labels), Some("web000@kata".to_string()));
    }

    #[test]
    fn test_wo_regex() {
        let labels = get_labels();
        let rule = Eval {
            source_labels: vec!["server".to_string(), "subsystem".to_string()],
            separator: "@".into(),
            regex: None,
            replacement: Some("@--->$0".into()),
        };
        assert_eq!(rule.apply(&labels), Some("@--->web000@kata".to_string()));
    }
    #[test]
    fn test_regex_mismatch() {
        let labels = get_labels();
        let rule = Eval {
            source_labels: vec!["server".to_string(), "subsystem".to_string()],
            separator: "@".into(),
            regex: Some(Regex::new("@bata").unwrap()),
            replacement: None,
        };
        assert_eq!(rule.apply(&labels), None);
    }
    #[test]
    fn test_regex() {
        let labels = get_labels();
        let rule = Eval {
            source_labels: vec!["server".to_string(), "subsystem".to_string()],
            separator: "@".into(),
            regex: Some(Regex::new("@ka").unwrap()),
            replacement: None,
        };
        assert_eq!(rule.apply(&labels), Some("web000@kata".to_string()));
    }
    #[test]
    fn test_regex_and_replacement() {
        let labels = get_labels();
        let rule = Eval {
            source_labels: vec!["server".to_string(), "subsystem".to_string()],
            separator: "@".into(),
            regex: Some(Regex::new("web(\\d+)@ka(.+)").unwrap()),
            replacement: Some("$0-->$1->$2".into()),
        };
        assert_eq!(
            rule.apply(&labels),
            Some("web000@kata-->000->ta".to_string())
        );
    }
}
