use crate::abilities;
use crate::effects;
use crate::rules;
use crate::statics;

pub fn instead<E: rules::traits::event::Event>(
    would: statics::Would<E>,
    then: impl Into<rules::Then>,
) -> abilities::Continuous {
    abilities::Continuous {
        filter: would.filter,
        as_long_as: None,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::event::<E>(),
            change: rules::Change::Instead {
                holder: would.holder,
                when: would.when,
                then: then.into(),
            },
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::creature;
    use crate::effects;
    use crate::events;
    use crate::rules;
    use crate::statics;

    use super::instead;

    #[test]
    fn test_usage() {
        let ability = instead(
            statics::Would::<events::DrawFromEmptyLibrary>::of(creature()),
            vec![rules::EventTemplate::WinGame {
                who: events::DrawFromEmptyLibrary::PLAYER.into(),
            }],
        );
        assert_eq!(ability.filter, creature());
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::event::<events::DrawFromEmptyLibrary>(),
                change: rules::Change::Instead {
                    holder: events::DrawFromEmptyLibrary::PLAYER.into(),
                    when: rules::Condition::True,
                    then: rules::Then::Events(vec![rules::EventTemplate::WinGame {
                        who: events::DrawFromEmptyLibrary::PLAYER.into(),
                    }]),
                },
            })
        );
    }

    #[test]
    fn bare_template_is_not_optional() {
        let ability = instead(
            statics::Would::<events::DrawFromEmptyLibrary>::of(creature()),
            rules::EventTemplate::WinGame {
                who: events::DrawFromEmptyLibrary::PLAYER.into(),
            },
        );
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::event::<events::DrawFromEmptyLibrary>(),
                change: rules::Change::Instead {
                    holder: events::DrawFromEmptyLibrary::PLAYER.into(),
                    when: rules::Condition::True,
                    then: rules::Then::Events(vec![rules::EventTemplate::WinGame {
                        who: events::DrawFromEmptyLibrary::PLAYER.into(),
                    }]),
                },
            })
        );
    }
}
