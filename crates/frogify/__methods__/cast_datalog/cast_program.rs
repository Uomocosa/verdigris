use frogmyr::Atom;
use frogmyr::Literal;
use frogmyr::Program;
use frogmyr::Rule;
use frogmyr::Term;
use frogmyr::Value;
use verdigris_rule_engine::events;
use verdigris_rule_engine::rules;
use verdigris_rule_engine::rules::traits::event::Event;

use crate::CastDatalog;
use crate::PlayDatalog;
use crate::TranslationError;

pub fn cast_program() -> Result<Program, TranslationError> {
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
    for restriction in events::Cast::restrictions() {
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
                    Term::Val(Value::Sym(CastDatalog::CAST)),
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
                    Term::Val(Value::Sym(CastDatalog::CAST)),
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

    use super::cast_program;

    #[test]
    fn test_usage() {
        let program = cast_program().unwrap();
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
    }

    #[test]
    fn building_the_program_twice_gives_equal_programs() {
        assert_eq!(cast_program().unwrap(), cast_program().unwrap());
    }
}
