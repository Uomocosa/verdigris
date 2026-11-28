use frogmyr::Atom;
use frogmyr::Literal;
use frogmyr::Program;
use frogmyr::Rule;
use frogmyr::Term;
use frogmyr::Value;
use verdigris_rule_engine::events;
use verdigris_rule_engine::rules;
use verdigris_rule_engine::rules::traits::event::Event;

use crate::AttackDatalog;
use crate::PlayDatalog;
use crate::TranslationError;

pub fn attack_program() -> Result<Program, TranslationError> {
    let candidate = Atom {
        rel: AttackDatalog::ATTACK_CANDIDATE,
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
    let card = rules::ArgRef::Arg(events::Attack::ATTACKER);
    let holder = rules::ArgRef::Arg(events::Attack::ATTACKER);
    let mut next_hold = 0;
    for restriction in events::Attack::restrictions() {
        let root = PlayDatalog::translate_holds(
            &restriction.condition,
            &card,
            &holder,
            &candidate,
            &mut program_rules,
            &mut next_hold,
        )?;
        let mut body = vec![Literal::Pos(candidate.clone()), Literal::Neg(root)];
        if !restriction.unwaivable {
            body.push(Literal::Neg(Atom {
                rel: PlayDatalog::WAIVED,
                terms: vec![
                    Term::Val(Value::Sym(AttackDatalog::ATTACK)),
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
                    Term::Val(Value::Sym(AttackDatalog::ATTACK)),
                    Term::Var(PlayDatalog::CARD_VAR),
                    Term::Var(PlayDatalog::PLAYER_VAR),
                    Term::Val(Value::Sym(*restriction.id)),
                ],
            },
            body,
        });
    }
    program_rules.push(Rule {
        head: Atom {
            rel: PlayDatalog::VIOLATES,
            terms: vec![
                Term::Val(Value::Sym(AttackDatalog::ATTACK)),
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
    use verdigris_rule_engine::events;
    use verdigris_rule_engine::rules::traits::event::Event;

    use super::attack_program;
    use crate::testing;

    #[test]
    fn test_usage() {
        let program = attack_program().unwrap();
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
    }

    #[test]
    fn building_the_program_twice_gives_equal_programs() {
        assert_eq!(attack_program().unwrap(), attack_program().unwrap());
    }
}
