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
        Ok(KeepRule {
            eval: Eval::try_from(value)?,
        })
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
