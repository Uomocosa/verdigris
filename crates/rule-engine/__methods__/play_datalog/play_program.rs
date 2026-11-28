use crate::events;
use crate::rules;
use frogmyr::Atom;
use frogmyr::Literal;
use frogmyr::Program;
use frogmyr::Rule;
use frogmyr::Term;
use frogmyr::Value;

use crate::legality;

pub fn play_program(
    answers: &rules::Rules,
    event: &events::Play,
) -> Result<Program, legality::TranslationError> {
    let candidate = Atom {
        rel: legality::PlayDatalog::PLAY_CANDIDATE,
        terms: vec![
            Term::Var(legality::PlayDatalog::CARD_VAR),
            Term::Var(legality::PlayDatalog::PLAYER_VAR),
        ],
    };
    let mut program_rules = vec![Rule {
        head: candidate.clone(),
        body: vec![
            Literal::Pos(Atom {
                rel: legality::PlayDatalog::OBJECT,
                terms: vec![Term::Var(legality::PlayDatalog::CARD_VAR)],
            }),
            Literal::Pos(Atom {
                rel: legality::PlayDatalog::PLAYER_FACT,
                terms: vec![Term::Var(legality::PlayDatalog::PLAYER_VAR)],
            }),
        ],
    }];
    let card = rules::ArgRef::Arg(events::Play::CARD);
    let holder = rules::ArgRef::Arg(events::Play::PLAYER);
    let mut next_hold = 0;
    legality::PlayDatalog::base_restriction_rules(
        answers,
        event,
        &candidate,
        &card,
        &holder,
        legality::PlayDatalog::PLAY,
        &mut program_rules,
        &mut next_hold,
    )?;
    legality::PlayDatalog::card_restriction_rules(
        answers,
        event,
        &candidate,
        &card,
        &holder,
        legality::PlayDatalog::PLAY,
        &mut program_rules,
        &mut next_hold,
    )?;
    program_rules.push(Rule {
        head: Atom {
            rel: legality::PlayDatalog::VIOLATES,
            terms: vec![
                Term::Val(Value::Sym(legality::PlayDatalog::PLAY)),
                Term::Var(legality::PlayDatalog::CARD_VAR),
                Term::Var(legality::PlayDatalog::PLAYER_VAR),
                Term::Var("I"),
            ],
        },
        body: vec![
            Literal::Pos(candidate),
            Literal::Pos(Atom {
                rel: legality::PlayDatalog::ADDED,
                terms: vec![
                    Term::Var(legality::PlayDatalog::CARD_VAR),
                    Term::Var(legality::PlayDatalog::PLAYER_VAR),
                    Term::Var("I"),
                ],
            }),
            Literal::Neg(Atom {
                rel: legality::PlayDatalog::WAIVED,
                terms: vec![
                    Term::Val(Value::Sym(legality::PlayDatalog::PLAY)),
                    Term::Var(legality::PlayDatalog::CARD_VAR),
                    Term::Var(legality::PlayDatalog::PLAYER_VAR),
                    Term::Var("I"),
                ],
            }),
        ],
    });
    Ok(Program {
        rules: program_rules,
    }
    .canonical())
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::events;
    use crate::players;
    use crate::rules::traits::event::Event;
    use crate::testing::FOREST;
    use crate::testing::MockGame;
    use crate::testing::P1;
    use crate::testing::P2;

    use super::play_program;

    #[test]
    fn test_usage() {
        let (mut game, [p1, _p2]) = MockGame::new([P1.hand([FOREST]), P2]);
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST).unwrap();
        game.read_rules(|answers, _legal| {
            let event = events::Play {
                card: *forest,
                player: players::PlayerId(*p1),
            };
            let program = play_program(answers, &event).unwrap();
            assert_eq!(program.rules.len(), 18);
            assert!(program.rules.iter().any(|rule| *rule.head.rel == "play_candidate"));
            assert_eq!(
                program.rules.iter().filter(|rule| *rule.head.rel == "violates").count(),
                9,
                "one violates rule per restriction plus the card-side veto"
            );
            assert_eq!(
                program.rules.iter().filter(|rule| rule.head.rel.starts_with("holds_")).count(),
                8,
                "one holds rule per single-atom Play restriction"
            );
            assert_eq!(
                events::Play::restrictions().len(),
                8,
                "Play gained a restriction; the program must cover it"
            );
        })
        .unwrap();
    }

    #[test]
    fn building_the_program_twice_gives_equal_programs() {
        let (mut game, [p1, _p2]) = MockGame::new([P1.hand([FOREST]), P2]);
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST).unwrap();
        game.read_rules(|answers, _legal| {
            let event = events::Play {
                card: *forest,
                player: players::PlayerId(*p1),
            };
            assert_eq!(
                play_program(answers, &event).unwrap(),
                play_program(answers, &event).unwrap()
            );
        })
        .unwrap();
    }
}
