#![cfg_attr(test, allow(clippy::panic_in_result_fn))]
pub mod actions;
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

pub use rules_plugin::RulesPlugin;

mod constants;
pub use constants::{
    ANY, ANY_TARGET, BEGINNING_OF_COMBAT, CLEANUP, COMBAT_DAMAGE, CREATURE, CREATURE_SPELL,
    DECLARE_ATTACKERS, DECLARE_BLOCKERS, DRAW, END, END_OF_COMBAT, EXILE, INSTANT_SPELL, MAIN_1,
    MAIN_2, NONBLACK_CREATURE, NONCREATURE_SPELL, OPPONENT, PLAYER, SPELL, UNTAP, UPKEEP, YOU,
};
pub use rules::{BATTLEFIELD, GRAVEYARD, HAND, LIBRARY, MAX_HAND_SIZE, STACK};

#[path = "../__methods__/mod.rs"]
mod methods;
