use crate::abilities;
use crate::effects;

#[must_use]
pub fn whenever(
    trigger: impl Into<abilities::Trigger>,
    effect: effects::Effect,
) -> abilities::TriggeredAbility {
    let trigger = trigger.into();
    abilities::TriggeredAbility {
        event: trigger.event,
        intervening_if: trigger.intervening_if,
        effect,
    }
}

#[must_use]
pub fn at(
    trigger: impl Into<abilities::Trigger>,
    effect: effects::Effect,
) -> abilities::TriggeredAbility {
    whenever(trigger, effect)
}

#[must_use]
pub fn when(
    trigger: impl Into<abilities::Trigger>,
    effect: effects::Effect,
) -> abilities::TriggeredAbility {
    whenever(trigger, effect)
}

#[must_use]
pub const fn when_this_dies() -> abilities::TriggerEvent {
    abilities::TriggerEvent::Dies
}
