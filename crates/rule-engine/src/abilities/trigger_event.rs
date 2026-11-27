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
    DealtCombatDamageToPlayer,
    Dies,
}

impl TriggerEvent {
    #[must_use]
    pub const fn beginning_of_next_cleanup() -> Self {
        Self::BeginningOfNextCleanup
    }

    #[must_use]
    pub const fn attacks() -> Self {
        Self::Attacks
    }

    #[must_use]
    pub const fn blocks() -> Self {
        Self::Blocks
    }

    #[must_use]
    pub const fn becomes_blocked() -> Self {
        Self::BecomesBlocked
    }

    #[must_use]
    pub const fn dealt_combat_damage_to_player() -> Self {
        Self::DealtCombatDamageToPlayer
    }
}

#[atomic_delegates]
impl TriggerEvent {
    #[must_use]
    pub const fn if_(self, condition: rules::Condition) -> abilities::Trigger {}
}

#[cfg(test)]
mod tests {
    use crate::UPKEEP;
    use crate::YOU;
    use crate::abilities;
    use crate::rules;

    #[test]
    fn test_usage() {
        let trigger = abilities::TriggerEvent::BeginningOfStep(UPKEEP, YOU)
            .if_(rules::life().of(rules::YOU).is(rules::Value(40).or_more()));
        assert_eq!(
            trigger,
            abilities::Trigger {
                event: abilities::TriggerEvent::BeginningOfStep(UPKEEP, YOU),
                intervening_if: Some(rules::life().of(rules::YOU).is(rules::Value(40).or_more())),
            }
        );
        assert_eq!(
            abilities::TriggerEvent::beginning_of_next_cleanup(),
            abilities::TriggerEvent::BeginningOfNextCleanup
        );
        assert_eq!(
            abilities::TriggerEvent::attacks(),
            abilities::TriggerEvent::Attacks
        );
        assert_eq!(
            abilities::TriggerEvent::blocks(),
            abilities::TriggerEvent::Blocks
        );
        assert_eq!(
            abilities::TriggerEvent::becomes_blocked(),
            abilities::TriggerEvent::BecomesBlocked
        );
        assert_eq!(
            abilities::TriggerEvent::dealt_combat_damage_to_player(),
            abilities::TriggerEvent::DealtCombatDamageToPlayer
        );
    }
}
