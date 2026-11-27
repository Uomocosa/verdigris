#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::components::{CastingStage, ChosenTarget, PendingTrigger, TriggeredAbilityOnStack};
pub use basic::messages::Resolution;
pub use basic::structs::ResolutionLookups;

mod trigger_queue;
pub use trigger_queue::TriggerQueue;

mod targeting;
pub use targeting::Targeting;
