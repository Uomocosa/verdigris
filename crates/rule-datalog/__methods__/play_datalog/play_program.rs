use verdigris_datalog_ir::Atom;
use verdigris_datalog_ir::Literal;
use verdigris_datalog_ir::Program;
use verdigris_datalog_ir::Rule;
use verdigris_datalog_ir::Term;
use verdigris_datalog_ir::Value;
use verdigris_rule_engine::events;
use verdigris_rule_engine::rules::traits::event::Event;

use crate::PlayDatalog;
use crate::TranslationError;

pub fn play_program() -> Result<Program, TranslationError> {
    let candidate = Atom {
        rel: PlayDatalog::PLAY_CANDIDATE,
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
    for restriction in events::Play::restrictions() {
        let mut body = vec![Literal::Pos(candidate.clone())];
        body.extend(PlayDatalog::translate_condition(&restriction.condition)?);
        if !restriction.unwaivable {
            body.push(Literal::Neg(Atom {
                rel: PlayDatalog::WAIVED,
                terms: vec![
                    Term::Val(Value::Sym(PlayDatalog::PLAY)),
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
                    Term::Val(Value::Sym(PlayDatalog::PLAY)),
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
                Term::Val(Value::Sym(PlayDatalog::PLAY)),
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

    use super::play_program;

    #[test]
    fn test_usage() {
        let program = play_program().unwrap();
        assert_eq!(program.rules.len(), 10);
        assert!(program.rules.iter().any(|rule| *rule.head.rel == "play_candidate"));
        assert_eq!(
            events::Play::restrictions().len(),
            8,
            "Play gained a restriction; the program must cover it"
        );
    }
}
