#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::condition_builders::{
    active_player, at_least, at_most, card_type, equal_to, event_field, has, has_priority, in_zone,
    is, main_phase, other, points_to, stack_empty, step_is,
};
pub use basic::enums::{Amount, Change, Characteristic, Comparison, Field, Legality, Role, may};
pub use basic::newtypes::{ComponentKey, EventId, RuleId};
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

mod restriction;
pub use restriction::Restriction;

mod restriction_id;
pub use restriction_id::RestrictionId;

pub mod land_plays;
pub use land_plays::LandPlays;

pub mod lethal_damage;
pub use lethal_damage::LethalDamage;

pub mod limit;
pub use limit::Limit;

pub mod maximum_hand_size;
pub use maximum_hand_size::MaximumHandSize;

mod condition;
pub use condition::Condition;

mod predicate_registry;
pub use predicate_registry::PredicateRegistry;

mod rules;
pub use rules::Rules;

mod rule_registry;
pub use rule_registry::RuleRegistry;

mod target;
pub use target::Target;

mod then;
pub use then::Then;
