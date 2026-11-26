#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::enums::{
    Amount, Change, Characteristic, Comparison, Field, Legality, Role, Target, may,
};
pub use basic::newtypes::{ClauseKey, ComponentKey, EventId, RuleId};
pub use basic::predicate_builders::{
    active_player, at_least, at_most, card_type, equal_to, event_field, has, has_priority, in_zone,
    is, main_phase, other, points_to, stack_empty, step_is,
};
pub use basic::structs::RuleChange;
pub use basic::type_aliases::TargetOf;

#[path = "__traits__/mod.rs"]
pub mod traits;

#[path = "__bevy_systems__/mod.rs"]
pub mod bevy_systems;

mod any_event;
pub use any_event::AnyEvent;

mod event_template;
pub use event_template::EventTemplate;

mod hook_queue;
pub use hook_queue::HookQueue;

mod hooked;
pub use hooked::Hooked;

mod clause;
pub use clause::Clause;

mod clause_id;
pub use clause_id::ClauseId;

pub mod land_plays;
pub use land_plays::LandPlays;

pub mod limit;
pub use limit::Limit;

pub mod maximum_hand_size;
pub use maximum_hand_size::MaximumHandSize;

mod predicate;
pub use predicate::Predicate;

mod predicate_registry;
pub use predicate_registry::PredicateRegistry;

mod rules;
pub use rules::Rules;

mod rule_registry;
pub use rule_registry::RuleRegistry;

mod then;
pub use then::Then;
