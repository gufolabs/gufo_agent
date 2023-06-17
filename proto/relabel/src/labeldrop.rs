// --------------------------------------------------------------------
// Gufo Agent: LabelDrop Rule
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use super::{ActionResult, ActiveLabels, RelabelRuleConfig, Relabeler};
use common::{AgentError, AgentResult};
use regex::Regex;

// Keeps matched label
#[derive(Debug)]
pub(crate) struct LabelDropRule {
    regex: Regex,
}

impl TryFrom<&RelabelRuleConfig> for LabelDropRule {
    type Error = AgentError;

    fn try_from(value: &RelabelRuleConfig) -> Result<Self, Self::Error> {
        // action must be `labeldrop`
        if let Some(x) = &value.action {
            if x != "labeldrop" {
                return Err(AgentError::ConfigurationError(
                    "'action' must be 'labeldrop'".to_string(),
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
        Ok(LabelDropRule { regex })
    }
}

impl Relabeler for LabelDropRule {
    fn apply(&self, active_labels: &mut ActiveLabels) -> AgentResult<ActionResult> {
        active_labels.retain(|k| !self.regex.is_match(k));
        Ok(ActionResult::Pass)
    }
}
