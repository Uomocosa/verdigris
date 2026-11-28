use crate::events;
use crate::rules;
use frogmyr::Atom;
use frogmyr::Literal;
use frogmyr::Program;
use frogmyr::Rule;
use frogmyr::Term;
use frogmyr::Value;

use crate::legality;

pub fn cast_program(
    answers: &rules::Rules,
    event: &events::Cast,
) -> Result<Program, legality::TranslationError> {
    let candidate = Atom {
        rel: legality::CastDatalog::CAST_CANDIDATE,
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
    let card = rules::ArgRef::Arg(events::Cast::SPELL);
    let holder = rules::ArgRef::Arg(events::Cast::CASTER);
    let mut next_hold = 0;
    legality::PlayDatalog::base_restriction_rules(
        answers,
        event,
        &candidate,
        &card,
        &holder,
        legality::CastDatalog::CAST,
        &mut program_rules,
        &mut next_hold,
    )?;
    legality::PlayDatalog::card_restriction_rules(
        answers,
        event,
        &candidate,
        &card,
        &holder,
        legality::CastDatalog::CAST,
        &mut program_rules,
        &mut next_hold,
    )?;
    program_rules.push(Rule {
        head: Atom {
            rel: legality::PlayDatalog::VIOLATES,
            terms: vec![
                Term::Val(Value::Sym(legality::CastDatalog::CAST)),
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
                    Term::Val(Value::Sym(legality::CastDatalog::CAST)),
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

    use super::cast_program;

    #[test]
    fn test_usage() {
        let (mut game, [p1, _p2]) = MockGame::new([P1.hand([FOREST]), P2]);
        let [spell] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST).unwrap();
        game.read_rules(|answers, _legal| {
            let event = events::Cast {
                spell: *spell,
                caster: players::PlayerId(*p1),
            };
            let program = cast_program(answers, &event).unwrap();
            assert_eq!(
                events::Cast::restrictions().len(),
                5,
                "Cast gained a restriction; the program must cover it"
            );
            assert!(program.rules.iter().any(|rule| *rule.head.rel == "cast_candidate"));
            assert_eq!(
                program.rules.iter().filter(|rule| *rule.head.rel == "violates").count(),
                6,
                "one violates rule per restriction plus the card-side veto"
            );
        })
        .unwrap();
    }

    #[test]
    fn building_the_program_twice_gives_equal_programs() {
        let (mut game, [p1, _p2]) = MockGame::new([P1.hand([FOREST]), P2]);
        let [spell] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST).unwrap();
        game.read_rules(|answers, _legal| {
            let event = events::Cast {
                spell: *spell,
                caster: players::PlayerId(*p1),
            };
            assert_eq!(
                cast_program(answers, &event).unwrap(),
                cast_program(answers, &event).unwrap()
            );
        })
        .unwrap();
    }
}
