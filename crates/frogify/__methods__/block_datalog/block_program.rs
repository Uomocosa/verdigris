use frogmyr::Atom;
use frogmyr::Literal;
use frogmyr::Program;
use frogmyr::Rule;
use frogmyr::Term;
use frogmyr::Value;
use verdigris_rule_engine::events;
use verdigris_rule_engine::rules;
use verdigris_rule_engine::rules::traits::event::Event;

use crate::BlockDatalog;
use crate::PlayDatalog;
use crate::TranslationError;

pub fn block_program(
    answers: &rules::Rules,
    event: &events::DeclareBlocker,
) -> Result<Program, TranslationError> {
    let candidate = Atom {
        rel: BlockDatalog::BLOCK_CANDIDATE,
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
                rel: PlayDatalog::OBJECT,
                terms: vec![Term::Var(PlayDatalog::PLAYER_VAR)],
            }),
        ],
    }];
    let card = rules::ArgRef::Arg(events::DeclareBlocker::BLOCKER);
    let holder = rules::ArgRef::Arg(events::DeclareBlocker::ATTACKER);
    let mut next_hold = 0;
    for restriction in events::DeclareBlocker::restrictions() {
        let root = PlayDatalog::translate_holds(
            &restriction.condition,
            &card,
            &holder,
            PlayDatalog::CARD_VAR,
            &candidate,
            &mut program_rules,
            &mut next_hold,
        )?;
        let mut body = vec![Literal::Pos(candidate.clone()), Literal::Neg(root)];
        if !restriction.unwaivable {
            body.push(Literal::Neg(Atom {
                rel: PlayDatalog::WAIVED,
                terms: vec![
                    Term::Val(Value::Sym(BlockDatalog::BLOCK)),
                    Term::Var(PlayDatalog::CARD_VAR),
                    Term::Var(PlayDatalog::PLAYER_VAR),
                    Term::Val(Value::Sym(*restriction.id)),
                ],
            }));
        }
        program_rules.push(Rule {
            head: Atom {
                rel: PlayDatalog::VIOLATES,
                terms: vec![
                    Term::Val(Value::Sym(BlockDatalog::BLOCK)),
                    Term::Var(PlayDatalog::CARD_VAR),
                    Term::Var(PlayDatalog::PLAYER_VAR),
                    Term::Val(Value::Sym(*restriction.id)),
                ],
            },
            body,
        });
    }
    PlayDatalog::card_restriction_rules(
        answers,
        event,
        &candidate,
        &card,
        &holder,
        BlockDatalog::BLOCK,
        &mut program_rules,
        &mut next_hold,
    )?;
    program_rules.push(Rule {
        head: Atom {
            rel: PlayDatalog::VIOLATES,
            terms: vec![
                Term::Val(Value::Sym(BlockDatalog::BLOCK)),
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
        ],
    });
    Ok(Program {
        rules: program_rules,
    }
    .canonical())
}

#[cfg(test)]
mod tests {
    use verdigris_rule_engine::BATTLEFIELD;
    use verdigris_rule_engine::events;
    use verdigris_rule_engine::rules::traits::event::Event;
    use verdigris_rule_engine::testing::GRIZZLY_BEARS;
    use verdigris_rule_engine::testing::MockGame;
    use verdigris_rule_engine::testing::P1;
    use verdigris_rule_engine::testing::P2;

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
