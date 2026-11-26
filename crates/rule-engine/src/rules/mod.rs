#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::condition_builders::{
    active_player, at_least, at_most, card_type, equal_to, event_field, has, has_priority, in_zone,
    is, main_phase, points_to, stack_empty, step_is,
};
pub use basic::enums::{Amount, Change, Characteristic, Comparison, Field, Legality, Role, may};
pub use basic::newtypes::{EventId, RuleId};
pub use basic::structs::RuleChange;
pub use basic::type_aliases::{Definition, Restriction};

mod arg_id;
pub use arg_id::ArgId;

mod arg_ref;
pub use arg_ref::ArgRef;

mod arg_value;
pub use arg_value::ArgValue;

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

mod statement;
pub use statement::Statement;

mod definition_id;
pub use definition_id::{DefinitionId, LETHAL_DAMAGE};

mod restriction_id;
pub use restriction_id::RestrictionId;

pub mod land_plays;
pub use land_plays::LandPlays;

pub mod limit;
pub use limit::Limit;

pub mod maximum_hand_size;
pub use maximum_hand_size::MaximumHandSize;

mod component_ref;
pub use component_ref::ComponentRef;

mod condition;
pub use condition::Condition;

mod rules;
pub use rules::Rules;

mod rule_registry;
pub use rule_registry::RuleRegistry;

mod target;
pub use target::Target;

mod then;
pub use then::Then;
