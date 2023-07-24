// --------------------------------------------------------------------
// Gufo Agent: LabelMap Rule
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use super::{ActionResult, ActiveLabels, RelabelRuleConfig, Relabeler};
use common::{AgentError, AgentResult};
use regex::Regex;

// Keeps matched label
#[derive(Debug)]
pub(crate) struct LabelMapRule {
    regex: Regex,
    replacement: String,
}

impl TryFrom<&RelabelRuleConfig> for LabelMapRule {
    type Error = AgentError;

    fn try_from(value: &RelabelRuleConfig) -> Result<Self, Self::Error> {
        // action must be `labelkeep`
        if let Some(x) = &value.action {
            if x != "labelmap" {
                return Err(AgentError::ConfigurationError(
                    "'action' must be 'labelmap'".to_string(),
                ));
            }
        }
        let regex = match &value.regex {
            Some(rx) => {
                Regex::new(rx).map_err(|e| AgentError::ConfigurationError(e.to_string()))?
            }
            None => {
                return Err(AgentError::ConfigurationError(
                    "'regex' must be set".to_string(),
                ))
            }
        };
        let replacement = match &value.replacement {
            Some(x) => x.clone(),
            None => {
                return Err(AgentError::ConfigurationError(
                    "'replacement' must be set".to_string(),
                ))
            }
        };
        Ok(LabelMapRule { regex, replacement })
    }
}

impl Relabeler for LabelMapRule {
    fn apply(&self, active_labels: &mut ActiveLabels) -> AgentResult<ActionResult> {
        active_labels.rename_if(|k| {
            self.regex.captures(k.as_str()).map(|caps| {
                let mut x = String::new();
                caps.expand(self.replacement.as_str(), &mut x);
                x
            })
        });
        Ok(ActionResult::Pass)
    }
}

#[cfg(test)]
mod tests {
    use super::{ActionResult, ActiveLabels, LabelMapRule, RelabelRuleConfig, Relabeler};
    use common::Label;

    #[test]
    fn test_invalid_action() {
        let yaml = r#"action: drop_something"#;
        let cfg = serde_yaml::from_str::<RelabelRuleConfig>(yaml).unwrap();
        assert!(LabelMapRule::try_from(&cfg).is_err());
    }
    #[test]
    fn test_match() {
        let yaml = r#"
        action: labelmap
        regex: a
        replacement: d
        "#;
        let cfg = serde_yaml::from_str::<RelabelRuleConfig>(yaml).unwrap();
        let rule = LabelMapRule::try_from(&cfg).unwrap();
        let mut labels = ActiveLabels::new(vec![
            Label::new("a", "1"),
            Label::new("b", "2"),
            Label::new("c", "3"),
        ]);
        assert_eq!(rule.apply(&mut labels).unwrap(), ActionResult::Pass);
        assert!(labels.get("a").is_none());
        assert_eq!(labels.get("b").unwrap(), "2");
        assert_eq!(labels.get("c").unwrap(), "3");
        assert_eq!(labels.get("d").unwrap(), "1");
    }
}
