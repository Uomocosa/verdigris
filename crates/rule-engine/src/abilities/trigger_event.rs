use atomic_delegate_macros::atomic_delegates;

use crate::abilities;
use crate::cards;
use crate::rules;
use crate::turns;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TriggerEvent {
    Enters(cards::ObjectFilter),
    BeginningOfStep(turns::Step, rules::Players),
    BeginningOfNextCleanup,
    Attacks,
    Blocks,
    BecomesBlocked,
    DealsDamage { combat: bool, to: rules::Players },
    Dies,
}

impl TriggerEvent {
    #[must_use]
    pub const fn beginning_of_next_cleanup() -> Self {
        Self::BeginningOfNextCleanup
    }
}

#[atomic_delegates]
impl TriggerEvent {
    #[must_use]
    pub const fn if_(self, condition: rules::Condition) -> abilities::Trigger {}
}

#[cfg(test)]
mod tests {
    use crate::PLAYER;
    use crate::UPKEEP;
    use crate::YOU;
    use crate::abilities;
    use crate::rules;

    #[test]
    fn test_usage() {
        let trigger = abilities::TriggerEvent::BeginningOfStep(UPKEEP, YOU.into())
            .if_(rules::life().of(YOU).is(rules::Value(40).or_more()));
        assert_eq!(
            trigger,
            abilities::Trigger {
                event: abilities::TriggerEvent::BeginningOfStep(UPKEEP, YOU.into()),
                intervening_if: Some(rules::life().of(YOU).is(rules::Value(40).or_more())),
            }
        );
        assert_eq!(
            abilities::TriggerEvent::beginning_of_next_cleanup(),
            abilities::TriggerEvent::BeginningOfNextCleanup
        );
        assert_eq!(
            abilities::DealsDamage(true).to(rules::a(PLAYER)),
            abilities::TriggerEvent::DealsDamage {
                combat: true,
                to: rules::Players::All,
            }
        );
    }
}
