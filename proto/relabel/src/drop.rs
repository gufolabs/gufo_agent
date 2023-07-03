// --------------------------------------------------------------------
// Gufo Agent: Drop Rule
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use super::{ActionResult, ActiveLabels, Eval, RelabelRuleConfig, Relabeler};
use common::{AgentError, AgentResult};

// Drops matched label
#[derive(Debug)]
pub(crate) struct DropRule {
    eval: Eval,
}

impl TryFrom<&RelabelRuleConfig> for DropRule {
    type Error = AgentError;

    fn try_from(value: &RelabelRuleConfig) -> Result<Self, Self::Error> {
        // action must be `drop`
        if let Some(x) = &value.action {
            if x != "drop" {
                return Err(AgentError::ConfigurationError(
                    "'action' must be 'drop'".to_string(),
                ));
            }
        }
        // Parse
        let eval = Eval::try_from(value)?;
        // source_labels must be set
        eval.require_source_labels()?;
        //
        Ok(DropRule { eval })
    }
}

impl Relabeler for DropRule {
    fn apply(&self, active_labels: &mut ActiveLabels) -> AgentResult<ActionResult> {
        match self.eval.apply(active_labels) {
            Some(_) => Ok(ActionResult::Drop),
            None => Ok(ActionResult::Pass),
        }
    }
}
