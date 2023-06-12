// --------------------------------------------------------------------
// Gufo Agent: Active Labels
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use common::{AgentError, Label, Labels, Measure};
use std::collections::BTreeMap;

const NAME_LABEL: &str = "__name__";

#[derive(Default, Debug)]
pub(crate) struct ActiveLabels {
    labels: BTreeMap<String, String>,
}

impl TryFrom<(&Labels, &Labels, &Labels)> for ActiveLabels {
    type Error = AgentError;

    fn try_from(
        (agent_labels, collector_labels, measure_labels): (&Labels, &Labels, &Labels),
    ) -> Result<Self, Self::Error> {
        let mut r = ActiveLabels::default();
        agent_labels.update_map(&mut r.labels);
        collector_labels.update_map(&mut r.labels);
        measure_labels.update_map(&mut r.labels);
        Ok(r)
    }
}

impl ActiveLabels {
    #[inline]
    pub(crate) fn insert(&mut self, label: Label) {
        self.labels.insert(label.key.clone(), label.value);
    }
    #[inline]
    pub(crate) fn get(&self, name: &String) -> Option<&String> {
        self.labels.get(name)
    }
    #[inline]
    fn is_virtual(name: &String) -> bool {
        name == NAME_LABEL
    }
    pub(crate) fn to_measure(&self, measure: &Measure) -> Measure {
        let name = match self.labels.get(NAME_LABEL) {
            Some(x) => x,
            None => &measure.name, // __name__ has been dropped by rule
        };
        let labels = Labels::new(
            self.labels
                .iter()
                .filter(|k| !Self::is_virtual(k.0))
                .map(|(k, v)| Label::new(k, v))
                .collect(),
        );
        Measure {
            name: name.into(),
            help: measure.help.to_owned(),
            value: measure.value,
            labels,
        }
    }
    // Leave only virtual labels and labels matching function
    pub(crate) fn retain<F>(&mut self, f: F)
    where
        F: Fn(&String) -> bool,
    {
        self.labels
            .retain(|name, _| Self::is_virtual(name) || f(name));
    }
}
