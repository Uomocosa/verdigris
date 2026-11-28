#![cfg_attr(test, allow(clippy::panic_in_result_fn))]
pub mod abilities;
pub mod actions;
mod attempt;
#[path = "__basic__/mod.rs"]
mod basic;
pub mod cards;
#[path = "../__rules__/mod.rs"]
mod comprehensive_rules;
pub mod counters;
pub mod damage;
pub mod effects;
#[path = "../__events__/mod.rs"]
pub mod events;
mod game;
mod game_mut;
pub mod keywords;
pub mod players;
pub mod prelude;
pub mod rules;
mod rules_plugin;
pub mod stack;
pub mod statics;
#[cfg(feature = "testing")]
pub mod testing;
pub mod turns;
pub mod zones;

pub use attempt::Attempt;
pub use basic::enums::RuleSet;
pub use basic::messages::{Did, HooksDone, May, Resolve};
pub use basic::resources::GameEnded;
pub use basic::resources::StateBasedActionsPerformed;
pub use basic::schedules::StateBasedActions;
pub use game::Game;
pub use game_mut::GameMut;
pub use rules_plugin::RulesPlugin;

mod constants;
pub use constants::{
    ASSIGNS_AGAIN, BATTLEFIELD, BEGINNING_OF_COMBAT, CLEANUP, COMBAT_DAMAGE, DAMAGE_MARKED,
    DECLARE_ATTACKERS, DECLARE_BLOCKERS, DEFENDING_PLAYER, DRAW, END, END_OF_COMBAT, EXILE,
    FIRST_COMBAT_DAMAGE_STEP, GRAVEYARD, GRAVEYARD_ORDER, HAND, LEGEND, LIBRARY, LOSES_LIFE,
    MAIN_1, MAIN_2, MAX_STATE_BASED_PASSES, MINUS_ONE_MINUS_ONE, OPPONENT, ORDER_TRIGGERS, PLAYER,
    PLUS_ONE_PLUS_ONE, RESOLUTION_CAUSE, STACK, TARGET, THIS, UNTAP, UPKEEP, YOU,
};
pub use effects::{
    another_creature, any_target, attacking, controlled_by, creature, creature_spell, enchantment,
    instant_spell, nonblack_creature, noncreature_spell, player, spell, token,
};

#[path = "../__methods__/mod.rs"]
mod methods;
