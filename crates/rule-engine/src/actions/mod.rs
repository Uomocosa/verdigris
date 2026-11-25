#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::enums::{Action, Choice};
pub use basic::messages::{Answered, Ask, PlayerAction, Rejected};
pub use basic::resources::{AcceptedMays, AskQueue, PendingChoice, PendingMayAsks};
pub use basic::structs::{
    ArrangeGraveyardChoice, BlockAssignment, DamageAssignment, DiscardChoice, LegendaryChoice,
    MayChoice, OrderTriggersChoice, RequiredCombatDamage, TargetChoice,
};

#[path = "__bevy_systems__/mod.rs"]
pub mod bevy_systems;

mod combat_damage_choice;
pub use combat_damage_choice::CombatDamageChoice;
