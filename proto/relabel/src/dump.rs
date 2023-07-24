// --------------------------------------------------------------------
// Gufo Agent: Dump Rule
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use super::{ActionResult, ActiveLabels, RelabelRuleConfig, Relabeler};
use common::{AgentError, AgentResult};

// Keeps matched label
#[derive(Debug)]
pub(crate) struct DumpRule;

impl TryFrom<&RelabelRuleConfig> for DumpRule {
    type Error = AgentError;

    fn try_from(value: &RelabelRuleConfig) -> Result<Self, Self::Error> {
        // action must be `dump`
        if let Some(x) = &value.action {
            if x != "dump" {
                return Err(AgentError::ConfigurationError(
                    "'action' must be 'dump'".to_string(),
                ));
            }
        }
        Ok(DumpRule {})
    }
}

impl Relabeler for DumpRule {
    fn apply(&self, active_labels: &mut ActiveLabels) -> AgentResult<ActionResult> {
        log::info!("===[START OF LABELS]==========");
        for (k, v) in active_labels.iter() {
            log::info!("{} = {}", k, v);
        }
        log::info!("===[END OF LABELS]============");
        Ok(ActionResult::Pass)
    }
}

#[cfg(test)]
mod tests {
    use super::{ActionResult, ActiveLabels, DumpRule, RelabelRuleConfig, Relabeler};
    use common::Label;

    #[test]
    fn test_invalid_action() {
        let yaml = r#"action: drop_something"#;
        let cfg = serde_yaml::from_str::<RelabelRuleConfig>(yaml).unwrap();
        assert!(DumpRule::try_from(&cfg).is_err());
    }
    #[test]
    fn test_dump() {
        let yaml = r#"action: dump"#;
        let cfg = serde_yaml::from_str::<RelabelRuleConfig>(yaml).unwrap();
        let rule = DumpRule::try_from(&cfg).unwrap();
        let mut labels = ActiveLabels::new(vec![
            Label::new("a", "1"),
            Label::new("b", "1"),
            Label::new("c", "3"),
        ]);
        assert_eq!(rule.apply(&mut labels).unwrap(), ActionResult::Pass);
    }
}
