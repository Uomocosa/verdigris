use atomic_delegate_macros::atomic_delegates;

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
    pub const fn if_(self, condition: cards::Condition) -> cards::Trigger {}
}

#[cfg(test)]
mod tests {
    use crate::UPKEEP;
    use crate::YOU;
    use crate::cards;

    #[test]
    fn test_usage() {
        let trigger = cards::TriggerEvent::BeginningOfStep(UPKEEP, YOU)
            .if_(cards::Condition::LifeAtLeast(YOU, 40));
        assert_eq!(
            trigger,
            cards::Trigger {
                event: cards::TriggerEvent::BeginningOfStep(UPKEEP, YOU),
                intervening_if: Some(cards::Condition::LifeAtLeast(YOU, 40)),
            }
        );
    }
}
