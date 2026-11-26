use crate::abilities;
use crate::effects;
use crate::events;
use crate::rules;
use crate::statics;

pub fn cant_lose_the_game(all: statics::All) -> abilities::Continuous {
    const ID: rules::RestrictionId = rules::RestrictionId("statics/cant-lose-the-game");
    abilities::Continuous {
        filter: *all,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::event::<events::LoseGame>(),
            change: rules::Change::Restrict {
                holder: rules::Role::Player,
                restriction: ID.restriction("can’t lose the game", !rules::Condition::True),
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
    use crate::statics;

    use super::cant_lose_the_game;

    #[test]
    fn test_usage() {
        let ability = cant_lose_the_game(statics::all(PLAYER));
        assert_eq!(ability.filter, PLAYER);
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::event::<events::LoseGame>(),
                change: rules::Change::Restrict {
                    holder: rules::Role::Player,
                    restriction: rules::RestrictionId("statics/cant-lose-the-game")
                        .restriction("can’t lose the game", !rules::Condition::True,),
                },
            })
        );
    }
}
