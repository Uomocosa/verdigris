use crate::abilities;

pub const fn if_(
    event: abilities::TriggerEvent,
    condition: abilities::Condition,
) -> abilities::Trigger {
    abilities::Trigger {
        event,
        intervening_if: Some(condition),
    }
}

#[cfg(test)]
mod tests {
    use crate::UPKEEP;
    use crate::YOU;
    use crate::abilities;

    use super::if_;

    #[test]
    fn test_usage() {
        assert_eq!(
            if_(
                abilities::TriggerEvent::BeginningOfStep(UPKEEP, YOU),
                abilities::Condition::LifeAtLeast(YOU, 40)
            ),
            abilities::Trigger {
                event: abilities::TriggerEvent::BeginningOfStep(UPKEEP, YOU),
                intervening_if: Some(abilities::Condition::LifeAtLeast(YOU, 40)),
            }
        );
    }
}
