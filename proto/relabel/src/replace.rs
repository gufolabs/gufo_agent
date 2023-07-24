// --------------------------------------------------------------------
// Gufo Agent: Replace Rule
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use super::{ActionResult, ActiveLabels, Eval, RelabelRuleConfig, Relabeler};
use common::{AgentError, AgentResult, Label};

// Rewrites target_label with the result of eval
#[derive(Debug)]
pub(crate) struct ReplaceRule {
    eval: Eval,
    target_label: String,
}

impl TryFrom<&RelabelRuleConfig> for ReplaceRule {
    type Error = AgentError;

    fn try_from(value: &RelabelRuleConfig) -> Result<Self, Self::Error> {
        // action must be None or `replace`
        if let Some(x) = &value.action {
            if x != "replace" {
                return Err(AgentError::ConfigurationError(
                    "'action' must be 'replace'".to_string(),
                ));
            }
        }
        let target_label = match &value.target_label {
            Some(x) => x.clone(),
            None => {
                return Err(AgentError::ConfigurationError(
                    "'target_label' must be set".to_string(),
                ))
            }
        };
        Ok(ReplaceRule {
            eval: Eval::try_from(value)?,
            target_label,
        })
    }
}

impl Relabeler for ReplaceRule {
    fn apply(&self, active_labels: &mut ActiveLabels) -> AgentResult<ActionResult> {
        if let Some(v) = self.eval.apply(active_labels) {
            active_labels.insert(Label::new(self.target_label.clone(), v));
        }
        Ok(ActionResult::Pass)
    }
}

#[cfg(test)]
mod tests {
    use super::{ActionResult, ActiveLabels, RelabelRuleConfig, Relabeler, ReplaceRule};
    use common::Label;

    #[test]
    fn test_invalid_action() {
        let yaml = r#"action: drop_something"#;
        let cfg = serde_yaml::from_str::<RelabelRuleConfig>(yaml).unwrap();
        assert!(ReplaceRule::try_from(&cfg).is_err());
    }

    #[test]
    fn test_add() {
        let yaml = r#"
action: replace
target_label: z
replacement: 9
"#;
        let cfg = serde_yaml::from_str::<RelabelRuleConfig>(yaml).unwrap();
        let rule = ReplaceRule::try_from(&cfg).unwrap();
        let mut labels = ActiveLabels::new(vec![
            Label::new("a", "1"),
            Label::new("b", "2"),
            Label::new("c", "3"),
        ]);
        assert_eq!(rule.apply(&mut labels).unwrap(), ActionResult::Pass);
        assert_eq!(labels.get("z").unwrap(), "9");
    }
    #[test]
    fn test_match1() {
        let yaml = r#"
action: replace
source_labels: [a, b]
target_label: z
replacement: $0
"#;
        let cfg = serde_yaml::from_str::<RelabelRuleConfig>(yaml).unwrap();
        let rule = ReplaceRule::try_from(&cfg).unwrap();
        let mut labels = ActiveLabels::new(vec![
            Label::new("a", "1"),
            Label::new("b", "2"),
            Label::new("c", "3"),
        ]);
        assert_eq!(rule.apply(&mut labels).unwrap(), ActionResult::Pass);
        assert_eq!(labels.get("z").unwrap(), "1;2");
    }
}
