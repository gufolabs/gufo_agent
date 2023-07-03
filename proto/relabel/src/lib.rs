// --------------------------------------------------------------------
// Gufo Agent: Relabeling engine
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

pub(crate) mod config;
pub(crate) mod drop;
pub(crate) mod drop_if_equal;
pub(crate) mod dump;
pub(crate) mod eval;
pub(crate) mod keep;
pub(crate) mod labeldrop;
pub(crate) mod labelkeep;
pub(crate) mod labelmap;
pub(crate) mod labels;
pub(crate) mod replace;
pub(crate) mod ruleset;

pub use config::RelabelRuleConfig;
pub(crate) use drop::DropRule;
pub(crate) use drop_if_equal::DropIfEqualRule;
pub(crate) use dump::DumpRule;
pub(crate) use eval::Eval;
pub(crate) use keep::KeepRule;
pub(crate) use labeldrop::LabelDropRule;
pub(crate) use labelkeep::LabelKeepRule;
pub(crate) use labelmap::LabelMapRule;
pub use labels::ActiveLabels;
pub(crate) use replace::ReplaceRule;
pub use ruleset::{ActionResult, RelabelRuleset, Relabeler};
