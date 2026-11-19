pub mod actions;
pub mod cards;
pub mod effects;
mod rules_plugin;
#[cfg(feature = "testing")]
pub mod testing;
pub mod turns;
pub mod zones;

pub use rules_plugin::RulesPlugin;

mod constants;
pub use constants::{
    ANY, ANY_TARGET, BATTLEFIELD, CREATURE, EXILE, GRAVEYARD, HAND, LIBRARY, OPPONENT, STACK, YOU,
};

#[path = "../__methods__/mod.rs"]
mod methods;
