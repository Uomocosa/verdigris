use crate::rules;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause {
    pub key: rules::ClauseKey,
    pub text: &'static str,
    pub role: rules::Role,
    pub requires: rules::Predicate,
}
