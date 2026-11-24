#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::enums::{Amount, Change, Characteristic, Comparison, Field, Legality, Role, Target};
pub use basic::messages::{Proceeds, Would};
pub use basic::newtypes::{ClauseKey, ComponentKey, EventId, RuleId};
pub use basic::predicate_builders::{
    active_player, at_least, at_most, card_type, equal_to, event_field, has, has_priority, in_zone,
    main_phase, other, points_to, stack_empty, step_is,
};
pub use basic::structs::RuleChange;
pub use basic::type_aliases::TargetOf;

#[path = "__traits__/mod.rs"]
pub mod traits;

pub mod cast_spell;
pub use cast_spell::CastSpell;

mod any_event;
pub use any_event::AnyEvent;

mod create_tokens;
pub use create_tokens::CreateTokens;

mod counter;
pub use counter::Counter;

mod deal_damage;
pub use deal_damage::DealDamage;

mod destroy;
pub use destroy::Destroy;

mod discard;
pub use discard::Discard;

mod draw;
pub use draw::Draw;

mod enter_battlefield;
pub use enter_battlefield::EnterBattlefield;

mod event_template;
pub use event_template::EventTemplate;

mod gain_life;
pub use gain_life::GainLife;

mod lose_game;
pub use lose_game::LoseGame;

mod lose_life;
pub use lose_life::LoseLife;

mod mill;
pub use mill::Mill;

mod untap;
pub use untap::Untap;

mod clause;
pub use clause::Clause;

mod clause_id;
pub use clause_id::ClauseId;

pub mod declare_blocker;
pub use declare_blocker::DeclareBlocker;

mod draw_from_empty_library;
pub use draw_from_empty_library::DrawFromEmptyLibrary;

mod win_game;
pub use win_game::WinGame;

pub mod land_plays;
pub use land_plays::LandPlays;

pub mod limit;
pub use limit::Limit;

pub mod maximum_hand_size;
pub use maximum_hand_size::MaximumHandSize;

pub mod declare_attacker;
pub use declare_attacker::DeclareAttacker;

pub mod play_land;
pub use play_land::PlayLand;

mod predicate;
pub use predicate::Predicate;

mod predicate_registry;
pub use predicate_registry::PredicateRegistry;

mod put_into_graveyard;
pub use put_into_graveyard::PutIntoGraveyard;

mod rules;
pub use rules::Rules;

mod rule_registry;
pub use rule_registry::RuleRegistry;

mod tap_attacker;
pub use tap_attacker::TapAttacker;
