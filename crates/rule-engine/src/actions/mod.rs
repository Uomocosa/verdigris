mod action;
mod ask_queue;
#[path = "__basic__/mod.rs"]
mod basic;
mod choice;
mod pending_choice;

pub use action::Action;
pub use ask_queue::AskQueue;
pub use basic::messages::{Answered, Ask, DiscardRequest, PlayerAction, Rejected};
pub use basic::resources::{AcceptedMays, PendingMayAsks};
pub use basic::structs::{
    ArrangeGraveyardChoice, BlockAssignment, DamageAssignment, DefendingPlayerChoice,
    DiscardChoice, LegendaryChoice, MayChoice, OrderTriggersChoice, RequiredCombatDamage,
    TargetChoice,
};
pub use choice::Choice;
pub use pending_choice::PendingChoice;

#[path = "__bevy_systems__/mod.rs"]
pub mod bevy_systems;

mod combat_damage_choice;
pub use combat_damage_choice::CombatDamageChoice;
