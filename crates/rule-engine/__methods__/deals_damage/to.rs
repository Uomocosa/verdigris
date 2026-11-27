use crate::abilities;
use crate::rules;

pub fn to(
    deals: abilities::DealsDamage,
    recipient: impl Into<rules::An>,
) -> abilities::TriggerEvent {
    abilities::TriggerEvent::DealsDamage {
        combat: *deals,
        to: *recipient.into(),
    }
}

#[cfg(test)]
mod tests {
    use crate::PLAYER;
    use crate::abilities;
    use crate::rules;

    use super::to;

    #[test]
    fn test_usage() {
        assert_eq!(
            to(abilities::DealsDamage(true), rules::a(PLAYER)),
            abilities::TriggerEvent::DealsDamage {
                combat: true,
                to: rules::Players::All,
            }
        );
    }
}
