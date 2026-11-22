#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::card_text::{at, when, when_this_dies, whenever};
pub use basic::enums::Replacement;
pub use basic::structs::{Continuous, TriggeredAbility};

mod ability;
pub use ability::Ability;

mod condition;
pub use condition::Condition;

mod static_ability;
pub use static_ability::StaticAbility;

mod trigger;
pub use trigger::Trigger;

mod trigger_event;
pub use trigger_event::TriggerEvent;
