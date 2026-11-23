use crate::rules;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RuleChange {
    pub target: rules::Target,
    pub change: rules::Change,
}
