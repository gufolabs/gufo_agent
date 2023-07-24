// --------------------------------------------------------------------
// Gufo Agent: Relabeling ruleset
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use super::{
    ActiveLabels, DropIfEqualRule, DropRule, DumpRule, KeepRule, LabelDropRule, LabelKeepRule,
    LabelMapRule, RelabelRuleConfig, ReplaceRule,
};
use common::{AgentError, AgentResult, Label, Labels, Measure};

#[derive(Debug, PartialEq)]
pub enum ActionResult {
    Drop,
    Pass,
}

#[derive(Debug)]
pub(crate) enum RelabelRule {
    Replace(ReplaceRule),
    Keep(KeepRule),
    Drop(DropRule),
    DropIfEqual(DropIfEqualRule),
    Dump(DumpRule),
    LabelKeep(LabelKeepRule),
    LabelDrop(LabelDropRule),
    // HashMod
    LabelMap(LabelMapRule),
}

#[derive(Debug)]
pub struct RelabelRuleset {
    rules: Vec<RelabelRule>,
}

pub trait Relabeler {
    fn apply(&self, active_labels: &mut ActiveLabels) -> AgentResult<ActionResult>;
}

impl TryFrom<&RelabelRuleConfig> for RelabelRule {
    type Error = AgentError;

    fn try_from(value: &RelabelRuleConfig) -> Result<Self, Self::Error> {
        Ok(match &value.action {
            Some(action) => match action.as_str() {
                "drop" => RelabelRule::Drop(DropRule::try_from(value)?),
                "drop_if_equal" => RelabelRule::DropIfEqual(DropIfEqualRule::try_from(value)?),
                "dump" => RelabelRule::Dump(DumpRule::try_from(value)?),
                "labeldrop" => RelabelRule::LabelDrop(LabelDropRule::try_from(value)?),
                "labelkeep" => RelabelRule::LabelKeep(LabelKeepRule::try_from(value)?),
                "labelmap" => RelabelRule::LabelMap(LabelMapRule::try_from(value)?),
                "keep" => RelabelRule::Keep(KeepRule::try_from(value)?),
                "replace" => RelabelRule::Replace(ReplaceRule::try_from(value)?),
                _ => {
                    return Err(AgentError::ConfigurationError(format!(
                        "invalid action: {}",
                        action
                    )))
                }
            },
            None => RelabelRule::Replace(ReplaceRule::try_from(value)?),
        })
    }
}

impl TryFrom<&Vec<RelabelRuleConfig>> for RelabelRuleset {
    type Error = AgentError;

    fn try_from(value: &Vec<RelabelRuleConfig>) -> Result<Self, Self::Error> {
        Ok(RelabelRuleset {
            rules: value
                .iter()
                .map(RelabelRule::try_from)
                .collect::<AgentResult<Vec<_>>>()?,
        })
    }
}

impl Relabeler for RelabelRule {
    fn apply(&self, active_labels: &mut ActiveLabels) -> AgentResult<ActionResult> {
        match self {
            RelabelRule::Drop(rule) => rule.apply(active_labels),
            RelabelRule::DropIfEqual(rule) => rule.apply(active_labels),
            RelabelRule::Dump(rule) => rule.apply(active_labels),
            RelabelRule::LabelDrop(rule) => rule.apply(active_labels),
            RelabelRule::LabelKeep(rule) => rule.apply(active_labels),
            RelabelRule::LabelMap(rule) => rule.apply(active_labels),
            RelabelRule::Keep(rule) => rule.apply(active_labels),
            RelabelRule::Replace(rule) => rule.apply(active_labels),
        }
    }
}

impl Relabeler for RelabelRuleset {
    fn apply(&self, active_labels: &mut ActiveLabels) -> AgentResult<ActionResult> {
        for rule in self.rules.iter() {
            match rule.apply(active_labels)? {
                ActionResult::Drop => return Ok(ActionResult::Drop),
                ActionResult::Pass => continue,
            }
        }
        Ok(ActionResult::Pass)
    }
}

impl RelabelRuleset {
    pub fn process(
        &self,
        agent_labels: &Labels,
        collector_labels: &Labels,
        measure: &Measure,
    ) -> AgentResult<Option<Measure>> {
        let mut labels = ActiveLabels::try_from((agent_labels, collector_labels, &measure.labels))?;
        labels.insert(Label::new("__name__", measure.name.clone()));
        match self.apply(&mut labels)? {
            ActionResult::Pass => {
                // Replace
                Ok(Some(labels.to_measure(measure)))
            }
            ActionResult::Drop => {
                log::debug!("Measure dropped by rule");
                Ok(None)
            }
        }
    }
}
