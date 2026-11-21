#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::card_text::{at, beginning_of, when, whenever};
pub use basic::structs::TriggeredAbility;

mod ability;
pub use ability::Ability;

mod condition;
pub use condition::Condition;

mod trigger;
pub use trigger::Trigger;

mod trigger_event;
pub use trigger_event::TriggerEvent;
