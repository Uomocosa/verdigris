pub mod actions;
pub mod cards;
pub mod effects;
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
    ANY, ANY_TARGET, BATTLEFIELD, CREATURE, CREATURE_SPELL, EXILE, GRAVEYARD, HAND, INSTANT_SPELL,
    LIBRARY, NONBLACK_CREATURE, NONCREATURE_SPELL, OPPONENT, SPELL, STACK, YOU,
};

#[path = "../__methods__/mod.rs"]
mod methods;
