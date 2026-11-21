use crate::cards;

pub const fn if_(event: cards::TriggerEvent, condition: cards::Condition) -> cards::Trigger {
    cards::Trigger {
        event,
        intervening_if: Some(condition),
    }
}

#[cfg(test)]
mod tests {
    use crate::UPKEEP;
    use crate::YOU;
    use crate::cards;

    use super::if_;

    #[test]
    fn test_usage() {
        assert_eq!(
            if_(
                cards::TriggerEvent::BeginningOfStep(UPKEEP, YOU),
                cards::Condition::LifeAtLeast(YOU, 40)
            ),
            cards::Trigger {
                event: cards::TriggerEvent::BeginningOfStep(UPKEEP, YOU),
                intervening_if: Some(cards::Condition::LifeAtLeast(YOU, 40)),
            }
        );
    }
}
