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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Target {
    Event(rules::EventId),
    Clause(rules::ClauseKey),
    Value(rules::RuleId),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Change {
    AsThough,
    Allow(rules::Predicate),
    OnlyIf {
        role: rules::Role,
        condition: rules::Predicate,
    },
    Cant,
    Must,
    Set(u32),
    Increase(u32),
    Decrease(u32),
    NoMaximum,
    Instead {
        when: rules::Predicate,
        then: Vec<rules::EventTemplate>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Legality {
    Legal,
    Illegal,
}
