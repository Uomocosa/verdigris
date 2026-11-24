use derive_more::From;

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
pub enum Field {
    Amount,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Amount {
    Fixed(u32),
    FromEvent(Field),
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, From)]
pub enum Then {
    Events(Vec<rules::EventTemplate>),
    May(rules::EventTemplate),
}

#[must_use]
pub const fn may(then: rules::EventTemplate) -> Then {
    Then::May(then)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Change {
    AsThough,
    Allow(rules::Predicate),
    OnlyIf {
        role: rules::Role,
        condition: rules::Predicate,
    },
    Cant {
        role: rules::Role,
    },
    Must,
    Skip,
    Perform,
    Set(u32),
    Increase(u32),
    Decrease(u32),
    NoMaximum,
    Instead {
        when: rules::Predicate,
        then: Then,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Legality {
    Legal,
    Illegal,
}
