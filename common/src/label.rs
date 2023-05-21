// --------------------------------------------------------------------
// Gufo Agent: Label definitions
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use std::collections::BTreeMap;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Ord, PartialOrd)]
pub struct Label {
    pub key: String,
    pub value: String,
}

impl Label {
    pub fn new<K: ToString, V: ToString>(key: K, value: V) -> Label {
        Label {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Ord, PartialOrd, Default)]
pub struct Labels(Option<Vec<Label>>);

impl Labels {
    pub fn new(v: Vec<Label>) -> Self {
        Labels(Some(v))
    }
    pub fn len(&self) -> usize {
        match &self.0 {
            Some(x) => x.len(),
            None => 0,
        }
    }
    pub fn is_empty(&self) -> bool {
        match &self.0 {
            Some(x) => x.is_empty(),
            None => true,
        }
    }
    pub fn push(&mut self, v: Label) {
        if !v.is_empty() {
            match &mut self.0 {
                Some(x) => {
                    x.push(v);
                }
                None => {
                    self.0 = Some(vec![v]);
                }
            }
        }
    }
    //
    fn update_map(&self, map: &mut BTreeMap<String, String>) {
        if let Some(v) = &self.0 {
            for x in v.iter() {
                if !x.is_empty() {
                    map.insert(x.key.clone(), x.value.clone());
                }
            }
        }
    }
    // Merge 3 set of labels and return sorted summary
    pub fn merge_sort4(v1: &Labels, v2: &Labels, v3: &Labels, v4: &Labels) -> Labels {
        if v1.is_empty() && v2.is_empty() && v3.is_empty() && v4.is_empty() {
            return Labels::default();
        }
        let mut map = BTreeMap::new();
        v1.update_map(&mut map);
        v2.update_map(&mut map);
        v3.update_map(&mut map);
        v4.update_map(&mut map);
        Labels::new(map.iter().map(|(k, v)| Label::new(k, v)).collect())
    }
    pub fn to_openmetrics(&self) -> String {
        match &self.0 {
            Some(labels) => {
                let s: Vec<String> = labels
                    .iter()
                    .map(|x| format!("{}=\"{}\"", x.key, x.value))
                    .collect();
                s.join(",")
            }
            None => "".into(),
        }
    }
}

pub type LabelsConfig = Option<BTreeMap<String, String>>;

impl From<LabelsConfig> for Labels {
    fn from(value: LabelsConfig) -> Self {
        match value {
            Some(map) => Labels::new(
                map.iter()
                    .map(|(label, value)| Label::new(label.clone(), value.clone()))
                    .collect(),
            ),
            None => Labels::default(),
        }
    }
}
