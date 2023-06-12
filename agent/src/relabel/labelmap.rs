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
