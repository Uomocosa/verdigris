mod action;
mod ask_queue;
#[path = "__basic__/mod.rs"]
mod basic;
mod choice;
mod count;
mod pending_choice;
mod pick_choice;

pub use action::Action;
pub use ask_queue::AskQueue;
pub use basic::enums::ChoiceWhy;
pub use basic::messages::{Answered, Ask, DiscardRequest, PlayerAction, Rejected};
pub use basic::newtypes::ChoiceId;
pub use basic::resources::{AcceptedMays, LegalActions, PendingMayAsks};
pub use basic::structs::{
    BlockAssignment, DamageAssignment, OrderTriggersChoice, RequiredCombatDamage,
};
pub use choice::Choice;
pub use count::Count;
pub use pending_choice::PendingChoice;
pub use pick_choice::PickChoice;

#[path = "__bevy_systems__/mod.rs"]
pub mod bevy_systems;

mod combat_damage_choice;
pub use combat_damage_choice::CombatDamageChoice;
