#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::card_text::{
    additional_phase, counter, creates, destroy, discard, draw, exile, gain, gain_life, get,
    lose_life, mill, target, win_the_game,
};
pub use basic::components::ContinuousEffect;
pub use basic::enums::{Duration, EffectKind, Layered, Modification};
pub use basic::filter_builders::{
    another_creature, any_target, attacking, card, controlled_by, creature, creature_spell,
    enchantment, instant_spell, nonblack_creature, noncreature_spell, owned_by, player, spell,
    token,
};
pub use basic::messages::{KeywordGranted, PowerToughnessModified};

#[path = "__traits__/mod.rs"]
pub mod traits;

mod token_definition;
pub use basic::components::SourceTimestamp;
pub use target::Target;
pub use token_definition::TokenDefinition;

mod target;

mod timestamp;
pub use timestamp::Timestamp;

mod effect;
pub use effect::Effect;

mod deals_damage;
pub use deals_damage::DealsDamage;

mod filter;
pub use filter::Filter;

mod zone_ref;
pub use zone_ref::ZoneRef;
