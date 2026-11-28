use crate::abilities;
use crate::effects;
use crate::rules;
use crate::testing;

pub fn restriction<E: rules::traits::event::Event>(
    card: &testing::Card,
) -> Option<rules::RestrictionId> {
    let definition = (card.definition)();
    definition.text.iter().find_map(|ability| match ability {
        abilities::Ability::Static(abilities::StaticAbility::Continuous(continuous)) => {
            match &continuous.modification {
                effects::Modification::Rule(change)
                    if change.target == rules::Target::event::<E>() =>
                {
                    match &change.change {
                        rules::Change::Restrict { restriction, .. } => Some(restriction.id),
                        _ => None,
                    }
                }
                _ => None,
            }
        }
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use crate::events;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::testing;

    use super::restriction;

    #[test]
    fn test_usage() {
        assert_eq!(
            restriction::<events::DeclareBlocker>(&testing::TEST_SKY_COURT),
            Some(rules::RestrictionId {
                card: "test_sky_court",
                kind: "restriction",
                event: events::DeclareBlocker::ID,
            })
        );
        assert_eq!(
            restriction::<events::DeclareAttackers>(&testing::TEST_SKY_COURT),
            None,
        );
    }
}
