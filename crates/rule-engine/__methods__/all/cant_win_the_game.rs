use crate::abilities;
use crate::effects;
use crate::rules;
use crate::rules::traits::legality_rule::LegalityRule;
use crate::statics;

pub fn cant_win_the_game(all: statics::All) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        modification: effects::Modification::Rule(rules::RuleChange::Restriction {
            rule: rules::WinsTheGame::ID,
            condition: !rules::Predicate::True,
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::PLAYER;
    use crate::effects;
    use crate::rules;
    use crate::statics;

    use super::cant_win_the_game;

    #[test]
    fn test_usage() {
        let ability = cant_win_the_game(statics::all(PLAYER));
        assert_eq!(ability.filter, PLAYER);
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange::Restriction {
                rule: rules::RuleId("104.2"),
                condition: !rules::Predicate::True,
            })
        );
    }
}
