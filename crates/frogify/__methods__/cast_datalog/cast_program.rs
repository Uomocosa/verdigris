use frogmyr::Atom;
use frogmyr::Literal;
use frogmyr::Program;
use frogmyr::Rule;
use frogmyr::Term;
use frogmyr::Value;
use verdigris_rule_engine::events;
use verdigris_rule_engine::rules;

use crate::CastDatalog;
use crate::PlayDatalog;
use crate::TranslationError;

pub fn cast_program(
    answers: &rules::Rules,
    event: &events::Cast,
) -> Result<Program, TranslationError> {
    let candidate = Atom {
        rel: CastDatalog::CAST_CANDIDATE,
        terms: vec![
            Term::Var(PlayDatalog::CARD_VAR),
            Term::Var(PlayDatalog::PLAYER_VAR),
        ],
    };
    let mut program_rules = vec![Rule {
        head: candidate.clone(),
        body: vec![
            Literal::Pos(Atom {
                rel: PlayDatalog::OBJECT,
                terms: vec![Term::Var(PlayDatalog::CARD_VAR)],
            }),
            Literal::Pos(Atom {
                rel: PlayDatalog::PLAYER_FACT,
                terms: vec![Term::Var(PlayDatalog::PLAYER_VAR)],
            }),
        ],
    }];
    let card = rules::ArgRef::Arg(events::Cast::SPELL);
    let holder = rules::ArgRef::Arg(events::Cast::CASTER);
    let mut next_hold = 0;
    PlayDatalog::base_restriction_rules(
        answers,
        event,
        &candidate,
        &card,
        &holder,
        CastDatalog::CAST,
        &mut program_rules,
        &mut next_hold,
    )?;
    PlayDatalog::card_restriction_rules(
        answers,
        event,
        &candidate,
        &card,
        &holder,
        CastDatalog::CAST,
        &mut program_rules,
        &mut next_hold,
    )?;
    program_rules.push(Rule {
        head: Atom {
            rel: PlayDatalog::VIOLATES,
            terms: vec![
                Term::Val(Value::Sym(CastDatalog::CAST)),
                Term::Var(PlayDatalog::CARD_VAR),
                Term::Var(PlayDatalog::PLAYER_VAR),
                Term::Var("I"),
            ],
        },
        body: vec![
            Literal::Pos(candidate),
            Literal::Pos(Atom {
                rel: PlayDatalog::ADDED,
                terms: vec![
                    Term::Var(PlayDatalog::CARD_VAR),
                    Term::Var(PlayDatalog::PLAYER_VAR),
                    Term::Var("I"),
                ],
            }),
            Literal::Neg(Atom {
                rel: PlayDatalog::WAIVED,
                terms: vec![
                    Term::Val(Value::Sym(CastDatalog::CAST)),
                    Term::Var(PlayDatalog::CARD_VAR),
                    Term::Var(PlayDatalog::PLAYER_VAR),
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
    use verdigris_rule_engine::HAND;
    use verdigris_rule_engine::events;
    use verdigris_rule_engine::players;
    use verdigris_rule_engine::rules::traits::event::Event;
    use verdigris_rule_engine::testing::FOREST;
    use verdigris_rule_engine::testing::MockGame;
    use verdigris_rule_engine::testing::P1;
    use verdigris_rule_engine::testing::P2;

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
