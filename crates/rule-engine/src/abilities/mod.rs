#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::card_text::{
    at, begin_the_game_with_it_on_the_battlefield, opening_hand_action, tap_add, when,
    when_this_dies, whenever,
};
pub use basic::enums::{ActivationCost, Replacement};
pub use basic::structs::{ActivatedAbility, Continuous, TriggeredAbility};

mod ability;
pub use ability::Ability;

mod static_ability;
pub use static_ability::StaticAbility;

mod trigger;
pub use trigger::Trigger;

mod trigger_event;
pub use trigger_event::TriggerEvent;
