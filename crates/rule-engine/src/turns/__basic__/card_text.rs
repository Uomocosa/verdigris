use crate::abilities;
use crate::effects;
use crate::turns;

#[must_use]
pub const fn beginning_of(
    step: turns::Step,
    player: effects::PlayerRef,
) -> abilities::TriggerEvent {
    abilities::TriggerEvent::BeginningOfStep(step, player)
}
