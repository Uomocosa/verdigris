#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::components::{CastingStage, ChosenTarget, TriggeredAbilityOnStack};
pub use basic::messages::Resolution;
pub use basic::structs::{PendingTrigger, ResolutionLookups};

mod trigger_queue;
pub use trigger_queue::TriggerQueue;
