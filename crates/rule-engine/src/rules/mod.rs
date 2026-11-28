#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::condition_builders::{
    a, active_player, always, an, card_type, cards_in_hand, cards_in_library, color, controlled_by,
    count, counters, damage, each, enchanted, exactly, greater_than, has, has_keyword,
    has_priority, in_zone, is, land_play_available, less_than, life, main_phase, never, points_to,
    power, spell, stack_empty, step_is, subtype, supertype, toughness,
};
pub use basic::enums::{Amount, ArgKind, Change, Legality, Why, may};
pub use basic::newtypes::{An, EventId, ResultId, RuleId};
pub use basic::structs::RuleChange;
pub use basic::type_aliases::{Definition, Restriction};

mod value;
pub use value::Value;

mod arg_id;
pub use arg_id::ArgId;

mod comparison;
pub use comparison::Comparison;

mod arg_ref;
pub use arg_ref::ArgRef;
pub(crate) use arg_ref::YOU;

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

mod count;
pub use count::Count;

mod damage;
pub use damage::Damage;

mod damage_filter;
pub use damage_filter::{COMBAT_DAMAGE, DAMAGE, DamageFilter};

mod characteristic;
pub use characteristic::Characteristic;

mod characteristic_of;
pub use characteristic_of::CharacteristicOf;

mod rules;
pub use rules::Rules;

mod each;
pub use each::Each;

mod you;
pub use you::You;

mod player_quantity;
pub use player_quantity::PlayerQuantity;

mod players;
pub use players::Players;

mod player_quantity_of;
pub use player_quantity_of::PlayerQuantityOf;

mod subject;
pub use subject::Subject;

mod this;
pub use this::This;

mod rule_registry;
pub use rule_registry::RuleRegistry;

mod target;
pub use target::Target;

mod then;
pub use then::Then;
