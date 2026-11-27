use std::collections::BTreeSet;

use bevy::prelude::Entity;

use crate::actions;
use crate::effects;
use crate::events;
use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChoiceWhy {
    Duplicate,
    NotAllowed(Entity),
    TooFew { min: u32 },
    TooMany { max: u32 },
    MustOrderAll,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Candidates {
    Fixed(BTreeSet<Entity>),
    Matching {
        filter: effects::Filter,
        context: events::AbilityContext,
        check: actions::CandidateCheck,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CandidateCheck {
    None,
    BecomeTarget,
    Discard { cause: rules::EventId },
}
