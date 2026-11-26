use crate::abilities;
use crate::effects;
use crate::rules;
use crate::statics;

pub fn instead(
    all: statics::All,
    event: rules::EventId,
    holder: impl Into<rules::ArgRef>,
    when: rules::Condition,
    then: impl Into<rules::Then>,
) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::Event(event),
            change: rules::Change::Instead {
                holder: holder.into(),
                when,
                then: then.into(),
            },
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::PLAYER;
    use crate::YOU;
    use crate::effects;
    use crate::events;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::statics;

    use super::instead;

    #[test]
    fn test_usage() {
        let ability = instead(
            statics::you(),
            events::DrawFromEmptyLibrary::ID,
            events::DrawFromEmptyLibrary::PLAYER,
            rules::Condition::True,
            vec![rules::EventTemplate::WinGame {
                who: events::DrawFromEmptyLibrary::PLAYER.into(),
            }],
        );
        assert_eq!(ability.filter, PLAYER.controlled_by(YOU));
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
