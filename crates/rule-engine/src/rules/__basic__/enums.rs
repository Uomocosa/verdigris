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
    GreaterThan,
    LessThan,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Change {
    AsThough {
        holder: rules::Role,
    },
    Allow {
        holder: rules::Role,
        when: rules::Condition,
    },
    Restrict {
        holder: rules::Role,
        restriction: rules::Restriction,
    },
    Must,
    Skip,
    Perform,
    Set(u32),
    Increase(u32),
    Decrease(u32),
    NoMaximum,
    Instead {
        when: rules::Condition,
        then: rules::Then,
    },
    Hook {
        when: rules::Condition,
        then: rules::Then,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Legality {
    Legal,
    Illegal,
}

#[must_use]
pub const fn may(then: rules::EventTemplate) -> rules::Then {
    rules::Then::May(then)
}
