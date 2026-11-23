use crate::abilities;
use crate::effects;
use crate::rules;
use crate::statics;

pub fn instead(
    all: statics::All,
    event: rules::EventId,
    when: rules::Predicate,
    then: Vec<rules::EventTemplate>,
) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::Event(event),
            change: rules::Change::Instead { when, then },
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::PLAYER;
    use crate::YOU;
    use crate::effects;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::statics;

    use super::instead;

    #[test]
    fn test_usage() {
        let ability = instead(
            statics::all(PLAYER.controlled_by(YOU)),
            rules::DrawFromEmptyLibrary::ID,
            rules::Predicate::True,
            vec![rules::EventTemplate::WinGame {
                who: rules::Role::Player,
            }],
        );
        assert_eq!(ability.filter, PLAYER.controlled_by(YOU));
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::Event(rules::DrawFromEmptyLibrary::ID),
                change: rules::Change::Instead {
                    when: rules::Predicate::True,
                    then: vec![rules::EventTemplate::WinGame {
                        who: rules::Role::Player,
                    }],
                },
            })
        );
    }
}
