use crate::events;
use crate::rules;
use frogmyr::Atom;
use frogmyr::Literal;
use frogmyr::Program;
use frogmyr::Rule;
use frogmyr::Term;
use frogmyr::Value;

use crate::legality;

pub fn attack_program(
    answers: &rules::Rules,
    event: events::Attack,
) -> Result<Program, legality::TranslationError> {
    let candidate = Atom {
        rel: legality::AttackDatalog::ATTACK_CANDIDATE,
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
    let card = rules::ArgRef::Arg(events::Attack::ATTACKER);
    let holder = rules::ArgRef::Arg(events::Attack::ATTACKER);
    let mut next_hold = 0;
    legality::PlayDatalog::base_restriction_rules(
        answers,
        &event,
        &candidate,
        &card,
        &holder,
        legality::AttackDatalog::ATTACK,
        &mut program_rules,
        &mut next_hold,
    )?;
    legality::PlayDatalog::card_restriction_rules(
        answers,
        &event,
        &candidate,
        &card,
        &holder,
        legality::AttackDatalog::ATTACK,
        &mut program_rules,
        &mut next_hold,
    )?;
    program_rules.push(Rule {
        head: Atom {
            rel: legality::PlayDatalog::VIOLATES,
            terms: vec![
                Term::Val(Value::Sym(legality::AttackDatalog::ATTACK)),
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
                    Term::Val(Value::Sym(legality::AttackDatalog::ATTACK)),
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

    use super::attack_program;
    use crate::testing;

    #[test]
    fn test_usage() {
        let (mut game, [p1, _p2]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [attacker] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS).unwrap();
        game.read_rules(|answers, _legal| {
            let event = events::Attack {
                attacker: *attacker,
            };
            let program = attack_program(answers, event).unwrap();
            assert_eq!(
                events::Attack::restrictions().len(),
                4,
                "Attack gained a restriction; the program must cover it"
            );
            assert_eq!(program.rules.len(), 12);
            assert!(program.rules.iter().any(|rule| rule.head == testing::attack_candidate()));
            assert_eq!(
                program.rules.iter().filter(|rule| *rule.head.rel == "violates").count(),
                5,
                "one violates rule per restriction plus the card-side veto"
            );
            assert_eq!(
                program.rules.iter().filter(|rule| rule.head.rel.starts_with("holds_")).count(),
                6,
                "one holds rule per single-atom restriction, two per negated one"
            );
        })
        .unwrap();
    }

    #[test]
    fn building_the_program_twice_gives_equal_programs() {
        let (mut game, [p1, _p2]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [attacker] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS).unwrap();
        game.read_rules(|answers, _legal| {
            let event = events::Attack {
                attacker: *attacker,
            };
            assert_eq!(
                attack_program(answers, event).unwrap(),
                attack_program(answers, event).unwrap()
            );
        })
        .unwrap();
    }
}
