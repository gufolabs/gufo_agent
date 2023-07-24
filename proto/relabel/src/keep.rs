// --------------------------------------------------------------------
// Gufo Agent: Keep Rule
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use super::{ActionResult, ActiveLabels, Eval, RelabelRuleConfig, Relabeler};
use common::{AgentError, AgentResult};

// Keeps matched label
#[derive(Debug)]
pub(crate) struct KeepRule {
    eval: Eval,
}

impl TryFrom<&RelabelRuleConfig> for KeepRule {
    type Error = AgentError;

    fn try_from(value: &RelabelRuleConfig) -> Result<Self, Self::Error> {
        // action must be `drop`
        if let Some(x) = &value.action {
            if x != "keep" {
                return Err(AgentError::ConfigurationError(
                    "'action' must be 'keep'".to_string(),
                ));
            }
        }
        // Parse
        let eval = Eval::try_from(value)?;
        // source_labels must be set
        eval.require_source_labels()?;
        Ok(KeepRule { eval })
    }
}

impl Relabeler for KeepRule {
    fn apply(&self, active_labels: &mut ActiveLabels) -> AgentResult<ActionResult> {
        match self.eval.apply(active_labels) {
            Some(_) => Ok(ActionResult::Pass),
            None => Ok(ActionResult::Drop),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ActionResult, ActiveLabels, KeepRule, RelabelRuleConfig, Relabeler};
    use common::Label;

    #[test]
    fn test_invalid_action() {
        let yaml = r#"action: drop_something"#;
        let cfg = serde_yaml::from_str::<RelabelRuleConfig>(yaml).unwrap();
        assert!(KeepRule::try_from(&cfg).is_err());
    }

    #[test]
    fn test_no_source_labels() {
        let yaml = r#"action: keep"#;
        let cfg = serde_yaml::from_str::<RelabelRuleConfig>(yaml).unwrap();
        assert!(KeepRule::try_from(&cfg).is_err());
    }
    #[test]
    fn test_match() {
        let yaml = r#"
action: keep
source_labels: [a, b]
regex: "1;1"
"#;
        let cfg = serde_yaml::from_str::<RelabelRuleConfig>(yaml).unwrap();
        let rule = KeepRule::try_from(&cfg).unwrap();
        let mut labels = ActiveLabels::new(vec![
            Label::new("a", "1"),
            Label::new("b", "1"),
            Label::new("c", "3"),
        ]);
        assert_eq!(rule.apply(&mut labels).unwrap(), ActionResult::Pass);
    }
    #[test]
    fn test_not_match() {
        let yaml = r#"
action: keep
source_labels: [a, b]
regex: "1;1"
"#;
        let cfg = serde_yaml::from_str::<RelabelRuleConfig>(yaml).unwrap();
        let rule = KeepRule::try_from(&cfg).unwrap();
        let mut labels = ActiveLabels::new(vec![
            Label::new("a", "1"),
            Label::new("b", "2"),
            Label::new("c", "3"),
        ]);
        assert_eq!(rule.apply(&mut labels).unwrap(), ActionResult::Drop);
    }
}
