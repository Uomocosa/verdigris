use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Characteristic {
    Power,
    Toughness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Comparison {
    AtLeast,
    AtMost,
    EqualTo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Role {
    Subject,
    Other,
    Player,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RuleChange {
    AsThough(rules::ClauseKey),
    Allow(rules::ClauseKey, rules::Predicate),
    Restriction {
        rule: rules::RuleId,
        condition: rules::Predicate,
    },
    Requirement(rules::RuleId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Legality {
    Legal,
    Illegal,
}
