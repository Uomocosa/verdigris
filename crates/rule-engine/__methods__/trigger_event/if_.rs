use crate::abilities;
use crate::rules;

pub const fn if_(
    event: abilities::TriggerEvent,
    condition: rules::Condition,
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
    use crate::rules;

    use super::if_;

    #[test]
    fn test_usage() {
        assert_eq!(
            if_(
                abilities::TriggerEvent::BeginningOfStep(UPKEEP, YOU),
                rules::life().at_least(40).of(rules::YOU)
            ),
            abilities::Trigger {
                event: abilities::TriggerEvent::BeginningOfStep(UPKEEP, YOU),
                intervening_if: Some(rules::life().at_least(40).of(rules::YOU)),
            }
        );
    }
}
