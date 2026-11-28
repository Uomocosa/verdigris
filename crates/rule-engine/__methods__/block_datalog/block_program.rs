use crate::events;
use crate::rules;
use frogmyr::Atom;
use frogmyr::Literal;
use frogmyr::Program;
use frogmyr::Rule;
use frogmyr::Term;
use frogmyr::Value;

use crate::legality;

pub fn block_program(
    answers: &rules::Rules,
    event: &events::DeclareBlocker,
) -> Result<Program, legality::TranslationError> {
    let candidate = Atom {
        rel: legality::BlockDatalog::BLOCK_CANDIDATE,
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
                rel: legality::PlayDatalog::OBJECT,
                terms: vec![Term::Var(legality::PlayDatalog::PLAYER_VAR)],
            }),
        ],
    }];
    let card = rules::ArgRef::Arg(events::DeclareBlocker::BLOCKER);
    let holder = rules::ArgRef::Arg(events::DeclareBlocker::ATTACKER);
    let mut next_hold = 0;
    legality::PlayDatalog::base_restriction_rules(
        answers,
        event,
        &candidate,
        &card,
        &holder,
        legality::BlockDatalog::BLOCK,
        &mut program_rules,
        &mut next_hold,
    )?;
    legality::PlayDatalog::card_restriction_rules(
        answers,
        event,
        &candidate,
        &card,
        &holder,
        legality::BlockDatalog::BLOCK,
        &mut program_rules,
        &mut next_hold,
    )?;
    program_rules.push(Rule {
        head: Atom {
            rel: legality::PlayDatalog::VIOLATES,
            terms: vec![
                Term::Val(Value::Sym(legality::BlockDatalog::BLOCK)),
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
                    Term::Val(Value::Sym(legality::BlockDatalog::BLOCK)),
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
    use crate::BATTLEFIELD;
    use crate::events;
    use crate::rules::traits::event::Event;
    use crate::testing::GRIZZLY_BEARS;
    use crate::testing::MockGame;
    use crate::testing::P1;
    use crate::testing::P2;

    use super::block_program;
    use crate::testing;

    #[test]
    fn test_usage() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [attacker] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS).unwrap();
        let [blocker] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS).unwrap();
        game.read_rules(|answers, _legal| {
            let event = events::DeclareBlocker {
                blocker: *blocker,
                attacker: *attacker,
            };
            let program = block_program(answers, &event).unwrap();
            assert_eq!(
                events::DeclareBlocker::restrictions().len(),
                3,
                "DeclareBlocker gained a restriction; the program must cover it"
            );
            assert_eq!(program.rules.len(), 9);
            assert!(program.rules.iter().any(|rule| rule.head == testing::block_candidate()));
            assert_eq!(
                program.rules.iter().filter(|rule| *rule.head.rel == "violates").count(),
                4,
                "one violates rule per restriction plus the card-side veto"
            );
            assert_eq!(
                program.rules.iter().filter(|rule| rule.head.rel.starts_with("holds_")).count(),
                4,
                "one holds rule per single-atom restriction, two per negated one"
            );
        })
        .unwrap();
    }

    #[test]
    fn building_the_program_twice_gives_equal_programs() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [attacker] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS).unwrap();
        let [blocker] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS).unwrap();
        game.read_rules(|answers, _legal| {
            let event = events::DeclareBlocker {
                blocker: *blocker,
                attacker: *attacker,
            };
            assert_eq!(
                block_program(answers, &event).unwrap(),
                block_program(answers, &event).unwrap()
            );
        })
        .unwrap();
    }
}
