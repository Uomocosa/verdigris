use crate::rules;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Replacement {
    EntersTapped,
    Rule(rules::RuleChange),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActivationCost {
    Tap,
}
