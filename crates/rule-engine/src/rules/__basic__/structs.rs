use crate::rules;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RuleChange {
    pub target: rules::Target,
    pub change: rules::Change,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct You;
