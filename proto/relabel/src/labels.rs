// --------------------------------------------------------------------
// Gufo Agent: Active Labels
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use common::{AgentError, Label, Labels, Measure};
use std::collections::BTreeMap;

const NAME_LABEL: &str = "__name__";
const ADDRESS_LABEL: &str = "__address__";
const META_PREFIX: &str = "__meta_";

#[derive(Default, Debug)]
pub struct ActiveLabels {
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
    pub(crate) fn is_virtual(name: &String) -> bool {
        name == NAME_LABEL || name == ADDRESS_LABEL || name.starts_with(META_PREFIX)
    }
    pub fn to_measure(&self, measure: &Measure) -> Measure {
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
            timestamp: measure.timestamp,
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
    // Rename label. Keep virtual labels
    pub(crate) fn rename(&mut self, src: String, dst: String) {
        if let Some(value) = self.labels.get(&src) {
            if Self::is_virtual(&src) {
                // Copy
                self.labels.insert(dst, value.to_owned());
            } else {
                // Move
                if let Some(v) = self.labels.remove(&src) {
                    // Always Some because of .get()
                    // value is owned, so we need no .clone()
                    self.labels.insert(dst, v);
                }
            }
        }
    }
    // Rename label if function returns Some
    pub(crate) fn rename_if<F>(&mut self, f: F)
    where
        F: Fn(&String) -> Option<String>,
    {
        let mut remap = Vec::with_capacity(self.labels.len());
        for k in self.labels.keys() {
            if let Some(new_name) = f(k) {
                remap.push((k.to_owned(), new_name.to_owned()));
            }
        }
        // Actual renaming
        for (src, dst) in remap.drain(..) {
            self.rename(src, dst);
        }
    }
}
