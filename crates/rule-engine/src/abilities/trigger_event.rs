use atomic_delegate_macros::atomic_delegates;

use crate::abilities;
use crate::cards;
use crate::effects;
use crate::turns;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TriggerEvent {
    Enters(cards::ObjectFilter),
    BeginningOfStep(turns::Step, effects::PlayerRef),
}

#[atomic_delegates]
impl TriggerEvent {
    #[must_use]
    pub const fn if_(self, condition: abilities::Condition) -> abilities::Trigger {}
}

#[cfg(test)]
mod tests {
    use crate::UPKEEP;
    use crate::YOU;
    use crate::abilities;

    #[test]
    fn test_usage() {
        let trigger = abilities::TriggerEvent::BeginningOfStep(UPKEEP, YOU)
            .if_(abilities::Condition::LifeAtLeast(YOU, 40));
        assert_eq!(
            trigger,
            abilities::Trigger {
                event: abilities::TriggerEvent::BeginningOfStep(UPKEEP, YOU),
                intervening_if: Some(abilities::Condition::LifeAtLeast(YOU, 40)),
            }
        );
    }
}
