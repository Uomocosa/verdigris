use crate::abilities;
use crate::effects;
use crate::rules;

pub fn with_card(ability: abilities::Ability, card: &'static str) -> abilities::Ability {
    match ability {
        abilities::Ability::Static(abilities::StaticAbility::Continuous(mut continuous)) => {
            if let effects::Modification::Rule(rules::RuleChange {
                change: rules::Change::Restrict { restriction, .. },
                ..
            }) = &mut continuous.modification
                && restriction.id.card == rules::RestrictionId::UNSET
            {
                restriction.id.card = card;
            }
            abilities::Ability::Static(abilities::StaticAbility::Continuous(continuous))
        }
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use crate::PLAYER;
    use crate::abilities;
    use crate::effects;
    use crate::events;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::statics;

    use super::with_card;

    #[test]
    fn test_usage() {
        let ability = statics::all(PLAYER).cant::<events::Draw>(events::Draw::PLAYER);
        let stamped = with_card(abilities::Ability::from(ability.clone()), "test_card");
        assert_eq!(
            stamped,
            abilities::Ability::from(abilities::Continuous {
                filter: ability.filter,
                modification: effects::Modification::Rule(rules::RuleChange {
                    target: rules::Target::event::<events::Draw>(),
                    change: rules::Change::Restrict {
                        holder: events::Draw::PLAYER.into(),
                        restriction: rules::RestrictionId {
                            card: "test_card",
                            kind: "cant",
                            event: events::Draw::ID,
                        }
                        .restriction(!rules::Condition::True),
                    },
                }),
                as_long_as: None,
            })
        );
    }
}
