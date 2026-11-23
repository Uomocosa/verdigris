#![cfg_attr(test, allow(clippy::panic_in_result_fn))]
pub mod abilities;
pub mod actions;
#[path = "__basic__/mod.rs"]
mod basic;
pub mod cards;
#[path = "../__rules__/mod.rs"]
mod comprehensive_rules;
pub mod damage;
pub mod effects;
pub mod keywords;
pub mod players;
pub mod rules;
mod rules_plugin;
pub mod stack;
pub mod statics;
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
    CREATURE, CREATURE_SPELL, DECLARE_ATTACKERS, DECLARE_BLOCKERS, DRAW, ENCHANTMENT, END,
    END_OF_COMBAT, EXILE, GRAVEYARD, HAND, INSTANT_SPELL, LIBRARY, MAIN_1, MAIN_2,
    MAX_STATE_BASED_PASSES, NONBLACK_CREATURE, NONCREATURE_SPELL, OPPONENT, PLAYER, SPELL, STACK,
    TAP_ATTACKERS, THIS, UNTAP, UNTAP_STEP, UPKEEP, YOU,
};

#[path = "../__methods__/mod.rs"]
mod methods;
