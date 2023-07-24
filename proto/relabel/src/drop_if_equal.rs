// --------------------------------------------------------------------
// Gufo Agent: drop_if_equal Rule
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use super::{ActionResult, ActiveLabels, RelabelRuleConfig, Relabeler};
use common::{AgentError, AgentResult};

// Drops matched label
#[derive(Debug)]
pub(crate) struct DropIfEqualRule {
    source_labels: Vec<String>,
}

impl TryFrom<&RelabelRuleConfig> for DropIfEqualRule {
    type Error = AgentError;

    fn try_from(value: &RelabelRuleConfig) -> Result<Self, Self::Error> {
        // action must be `drop`
        if let Some(x) = &value.action {
            if x != "drop_if_equal" {
                return Err(AgentError::ConfigurationError(
                    "'action' must be 'drop_if_equal'".to_string(),
                ));
            }
        }
        let source_labels = match &value.source_labels {
            Some(x) => {
                // source_labels must contain at least 2 names
                if x.len() < 2 {
                    return Err(AgentError::ConfigurationError(
                        "'source_labels' must contain at least two names".to_string(),
                    ));
                }
                x.clone()
            }
            None => {
                return Err(AgentError::ConfigurationError(
                    "'source_labels' must be set".to_string(),
                ))
            }
        };
        Ok(DropIfEqualRule { source_labels })
    }
}

impl Relabeler for DropIfEqualRule {
    fn apply(&self, active_labels: &mut ActiveLabels) -> AgentResult<ActionResult> {
        let v = active_labels.get(&self.source_labels[0]);
        for n in self.source_labels[1..].iter() {
            if active_labels.get(n) != v {
                return Ok(ActionResult::Pass);
            }
        }
        Ok(ActionResult::Drop)
    }
}

#[cfg(test)]
mod tests {
    use super::{ActionResult, ActiveLabels, DropIfEqualRule, RelabelRuleConfig, Relabeler};
    use common::Label;

    #[test]
    fn test_invalid_action() {
        let yaml = r#"action: drop_something"#;
        let cfg = serde_yaml::from_str::<RelabelRuleConfig>(yaml).unwrap();
        assert!(DropIfEqualRule::try_from(&cfg).is_err());
    }

    #[test]
    fn test_no_source_labels() {
        let yaml = r#"action: drop_if_equal"#;
        let cfg = serde_yaml::from_str::<RelabelRuleConfig>(yaml).unwrap();
        assert!(DropIfEqualRule::try_from(&cfg).is_err());
    }
    #[test]
    fn test_short_source_labels() {
        let yaml = r#"
action: drop_if_equal
source_labels: [x]
"#;
        let cfg = serde_yaml::from_str::<RelabelRuleConfig>(yaml).unwrap();
        assert!(DropIfEqualRule::try_from(&cfg).is_err());
    }
    #[test]
    fn test_match2() {
        let yaml = r#"
action: drop_if_equal
source_labels: [a, b]
"#;
        let cfg = serde_yaml::from_str::<RelabelRuleConfig>(yaml).unwrap();
        let rule = DropIfEqualRule::try_from(&cfg).unwrap();
        let mut labels = ActiveLabels::new(vec![
            Label::new("a", "1"),
            Label::new("b", "1"),
            Label::new("c", "3"),
        ]);
        assert_eq!(rule.apply(&mut labels).unwrap(), ActionResult::Drop);
    }
    #[test]

    fn test_not_match2() {
        let yaml = r#"
action: drop_if_equal
source_labels: [a, b]
"#;
        let cfg = serde_yaml::from_str::<RelabelRuleConfig>(yaml).unwrap();
        let rule = DropIfEqualRule::try_from(&cfg).unwrap();
        let mut labels = ActiveLabels::new(vec![
            Label::new("a", "1"),
            Label::new("b", "2"),
            Label::new("c", "3"),
        ]);
        assert_eq!(rule.apply(&mut labels).unwrap(), ActionResult::Pass);
    }
    #[test]
    fn test_match3() {
        let yaml = r#"
action: drop_if_equal
source_labels: [a, b, c]
"#;
        let cfg = serde_yaml::from_str::<RelabelRuleConfig>(yaml).unwrap();
        let rule = DropIfEqualRule::try_from(&cfg).unwrap();
        let mut labels = ActiveLabels::new(vec![
            Label::new("a", "1"),
            Label::new("b", "1"),
            Label::new("c", "1"),
        ]);
        assert_eq!(rule.apply(&mut labels).unwrap(), ActionResult::Drop);
    }
    #[test]
    fn test_not_match3() {
        let yaml = r#"
action: drop_if_equal
source_labels: [a, b, c]
"#;
        let cfg = serde_yaml::from_str::<RelabelRuleConfig>(yaml).unwrap();
        let rule = DropIfEqualRule::try_from(&cfg).unwrap();
        let mut labels = ActiveLabels::new(vec![
            Label::new("a", "1"),
            Label::new("b", "1"),
            Label::new("c", "3"),
        ]);
        assert_eq!(rule.apply(&mut labels).unwrap(), ActionResult::Pass);
    }
}
