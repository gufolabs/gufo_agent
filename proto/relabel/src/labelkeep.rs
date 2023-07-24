// --------------------------------------------------------------------
// Gufo Agent: LabelKeep Rule
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use super::{ActionResult, ActiveLabels, RelabelRuleConfig, Relabeler};
use common::{AgentError, AgentResult};
use regex::Regex;

// Keeps matched label
#[derive(Debug)]
pub(crate) struct LabelKeepRule {
    regex: Regex,
}

impl TryFrom<&RelabelRuleConfig> for LabelKeepRule {
    type Error = AgentError;

    fn try_from(value: &RelabelRuleConfig) -> Result<Self, Self::Error> {
        // action must be `labelkeep`
        if let Some(x) = &value.action {
            if x != "labelkeep" {
                return Err(AgentError::ConfigurationError(
                    "'action' must be 'labelkeep'".to_string(),
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
        Ok(LabelKeepRule { regex })
    }
}

impl Relabeler for LabelKeepRule {
    fn apply(&self, active_labels: &mut ActiveLabels) -> AgentResult<ActionResult> {
        active_labels.retain(|k| self.regex.is_match(k));
        Ok(ActionResult::Pass)
    }
}

#[cfg(test)]
mod tests {
    use super::{ActionResult, ActiveLabels, LabelKeepRule, RelabelRuleConfig, Relabeler};
    use common::Label;

    #[test]
    fn test_invalid_action() {
        let yaml = r#"action: drop_something"#;
        let cfg = serde_yaml::from_str::<RelabelRuleConfig>(yaml).unwrap();
        assert!(LabelKeepRule::try_from(&cfg).is_err());
    }
    #[test]
    fn test_match() {
        let yaml = r#"
        action: labelkeep
        regex: a|b
        "#;
        let cfg = serde_yaml::from_str::<RelabelRuleConfig>(yaml).unwrap();
        let rule = LabelKeepRule::try_from(&cfg).unwrap();
        let mut labels = ActiveLabels::new(vec![
            Label::new("a", "1"),
            Label::new("b", "2"),
            Label::new("c", "3"),
        ]);
        assert_eq!(rule.apply(&mut labels).unwrap(), ActionResult::Pass);
        assert_eq!(labels.get("a").unwrap(), "1");
        assert_eq!(labels.get("b").unwrap(), "2");
        assert!(labels.get("c").is_none());
    }
}
