#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::enums::{Action, Choice};
pub use basic::messages::{PlayerAction, Rejected};
pub use basic::resources::PendingChoice;
pub use basic::structs::{
    BlockAssignment, DamageAssignment, DiscardChoice, OrderTriggersChoice, RequiredCombatDamage,
    TargetChoice,
};

mod combat_damage_choice;
pub use combat_damage_choice::CombatDamageChoice;
