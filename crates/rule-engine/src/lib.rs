#![cfg_attr(test, allow(clippy::panic_in_result_fn))]
pub mod actions;
#[path = "__basic__/mod.rs"]
mod basic;
pub mod cards;
pub mod damage;
pub mod effects;
pub mod players;
#[path = "../__rules__/mod.rs"]
mod rules;
mod rules_plugin;
pub mod stack;
#[cfg(feature = "testing")]
pub mod testing;
pub mod turns;
pub mod zones;

pub use basic::enums::RuleSet;
pub use basic::resources::StateBasedActionsPerformed;
pub use basic::schedules::StateBasedActions;
pub use rules_plugin::RulesPlugin;

mod constants;
pub use constants::{
    ANOTHER_CREATURE, ANY, ANY_TARGET, BATTLEFIELD, BEGINNING_OF_COMBAT, CLEANUP, COMBAT_DAMAGE,
    CREATURE, CREATURE_SPELL, DECLARE_ATTACKERS, DECLARE_BLOCKERS, DRAW, END, END_OF_COMBAT, EXILE,
    GRAVEYARD, HAND, INSTANT_SPELL, LIBRARY, MAIN_1, MAIN_2, MAX_HAND_SIZE, MAX_STATE_BASED_PASSES,
    NONBLACK_CREATURE, NONCREATURE_SPELL, OPPONENT, PLAYER, SPELL, STACK, UNTAP, UPKEEP, YOU,
};

#[path = "../__methods__/mod.rs"]
mod methods;
