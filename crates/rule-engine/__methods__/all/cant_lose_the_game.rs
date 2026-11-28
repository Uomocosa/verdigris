use crate::abilities;
use crate::effects;
use crate::events;
use crate::rules;
use crate::rules::traits::event::Event;
use crate::statics;

pub fn cant_lose_the_game(all: &statics::All) -> abilities::Continuous {
    abilities::Continuous {
        filter: all.into(),
        as_long_as: None,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::event::<events::LoseGame>(),
            change: rules::Change::Restrict {
                holder: events::LoseGame::PLAYER.into(),
                restriction: rules::RestrictionId {
                    card: rules::RestrictionId::UNSET,
                    kind: "cant",
                    event: events::LoseGame::ID,
                }
                .restriction(!rules::Condition::True),
            },
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::PLAYER;
    use crate::effects;
    use crate::events;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::statics;

    use super::cant_lose_the_game;

    #[test]
    fn test_usage() {
        let ability = cant_lose_the_game(&statics::all(PLAYER));
        assert_eq!(
            ability.filter,
            effects::Filter::player() & effects::Filter::controlled_by(PLAYER)
        );
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::event::<events::LoseGame>(),
                change: rules::Change::Restrict {
                    holder: events::LoseGame::PLAYER.into(),
                    restriction: rules::RestrictionId {
                        card: rules::RestrictionId::UNSET,
                        kind: "cant",
                        event: events::LoseGame::ID,
                    }
                    .restriction(!rules::Condition::True),
                },
            })
        );
    }
}
