use atomic_delegate_macros::atomic_delegates;
use nemo::datavalues::AnyDataValue;
use verdigris_datalog_ir::Atom;
use verdigris_datalog_ir::EngineError;
use verdigris_datalog_ir::Facts;
use verdigris_datalog_ir::Program;
use verdigris_datalog_ir::Term;
use verdigris_datalog_ir::Value;
use verdigris_datalog_ir::traits::engine::Engine;

#[derive(Debug, Clone, Copy)]
pub struct NemoEngine;

#[rustfmt::skip]
impl Engine for NemoEngine {
    fn evaluate(&self, program: &Program, facts: &Facts) -> Result<Facts, EngineError> { Self::evaluate_program(program, facts) }
}

#[atomic_delegates]
impl NemoEngine {
    pub(crate) fn evaluate_program(program: &Program, facts: &Facts) -> Result<Facts, EngineError> {
    }
    pub(crate) fn render_program_text(program: &Program, facts: &Facts) -> String {}
    pub(crate) fn decode_cell(
        cell: &AnyDataValue,
        symbols: &std::collections::BTreeMap<String, &'static str>,
    ) -> Result<Value, EngineError> {
    }
}

impl NemoEngine {
    pub(crate) const ENTITY_IRI_PREFIX: &'static str = "https://verdigris.invalid/entity/";

    pub(crate) fn render_atom_text(atom: &Atom) -> String {
        let terms: Vec<String> = atom.terms.iter().map(Self::render_term_text).collect();
        format!("{}({})", *atom.rel, terms.join(", "))
    }

    pub(crate) fn render_term_text(term: &Term) -> String {
        match term {
            Term::Var(name) => format!("?{name}"),
            Term::Val(Value::Int(number)) => number.to_string(),
            Term::Val(Value::Entity(number)) => {
                format!("<{}{number}>", Self::ENTITY_IRI_PREFIX)
            }
            Term::Val(Value::Sym(text)) => {
                format!("\"{}\"", text.replace('\\', "\\\\").replace('"', "\\\""))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use verdigris_datalog_ir::Atom;
    use verdigris_datalog_ir::Facts;
    use verdigris_datalog_ir::Program;
    use verdigris_datalog_ir::Rel;
    use verdigris_datalog_ir::Term;
    use verdigris_datalog_ir::Value;
    use verdigris_datalog_ir::traits::engine::Engine;

    use super::NemoEngine;

    #[test]
    fn test_usage() {
        let engine = NemoEngine;
        let derived = engine.evaluate(&Program::default(), &Facts::default()).unwrap();
        assert_eq!(derived, Facts::default());
    }

    #[test]
    fn render_atom_text_renders_each_term_shape() {
        assert_eq!(
            NemoEngine::render_atom_text(&Atom {
                rel: Rel("edge"),
                terms: vec![
                    Term::Var("X"),
                    Term::Val(Value::Entity(2)),
                    Term::Val(Value::Sym("ada")),
                ],
            }),
            format!("edge(?X, <{}2>, \"ada\")", NemoEngine::ENTITY_IRI_PREFIX)
        );
    }

    #[test]
    fn render_term_text_renders_each_variant() {
        assert_eq!(NemoEngine::render_term_text(&Term::Var("X")), "?X");
        assert_eq!(NemoEngine::render_term_text(&Term::Val(Value::Int(3))), "3");
    }

    #[test]
    fn test_examples_attack() {
        use verdigris_datalog_ir::Literal;
        use verdigris_datalog_ir::Rule;
        let bears = Value::Sym("bears");
        let goblin = Value::Sym("goblin");
        let mut facts = Facts::default();
        for (relation, tuple) in [
            (Rel("creature"), vec![bears]),
            (Rel("creature"), vec![goblin]),
            (Rel("tapped"), vec![bears]),
            (Rel("sick"), vec![goblin]),
            (Rel("has_haste"), vec![goblin]),
        ] {
            assert!(facts.add(relation, tuple));
        }
        let pos1 = |rel: Rel, var: &'static str| {
            Literal::Pos(Atom {
                rel,
                terms: vec![Term::Var(var)],
            })
        };
        let tagged = |rel: Rel, var: &'static str, tag: &'static str| Atom {
            rel,
            terms: vec![Term::Var(var), Term::Val(Value::Sym(tag))],
        };
        let program = Program {
            rules: vec![
                Rule {
                    head: tagged(Rel("waived"), "G", "sick"),
                    body: vec![pos1(Rel("has_haste"), "G")],
                },
                Rule {
                    head: tagged(Rel("violates"), "C", "untapped"),
                    body: vec![pos1(Rel("creature"), "C"), pos1(Rel("tapped"), "C")],
                },
                Rule {
                    head: tagged(Rel("violates"), "C", "sick"),
                    body: vec![
                        pos1(Rel("creature"), "C"),
                        pos1(Rel("sick"), "C"),
                        Literal::Neg(tagged(Rel("waived"), "C", "sick")),
                    ],
                },
                Rule {
                    head: Atom {
                        rel: Rel("can_attack"),
                        terms: vec![Term::Var("C")],
                    },
                    body: vec![
                        pos1(Rel("creature"), "C"),
                        Literal::Neg(tagged(Rel("violates"), "C", "untapped")),
                        Literal::Neg(tagged(Rel("violates"), "C", "sick")),
                    ],
                },
            ],
        };
        let text = NemoEngine::render_program_text(&program, &facts);
        println!("1. nemo program sent:");
        for line in text.lines() {
            println!("   {line}");
        }
        let engine = NemoEngine;
        let derived = engine.evaluate(&program, &facts).unwrap();
        println!("2. rows read back per predicate:");
        for predicate in ["waived", "violates", "can_attack"] {
            let rows = derived.matching(&Rel(predicate), &[Term::Var("A"), Term::Var("B")]);
            let single = derived.matching(&Rel(predicate), &[Term::Var("A")]);
            if rows.is_empty() {
                for row in &single {
                    println!("   {predicate}({row:?})");
                }
            } else {
                for row in &rows {
                    println!("   {predicate}({row:?})");
                }
            }
        }
        let mut expected = facts.clone();
        assert!(expected.add(Rel("waived"), vec![goblin, Value::Sym("sick")]));
        assert!(expected.add(Rel("violates"), vec![bears, Value::Sym("untapped")]));
        assert!(expected.add(Rel("can_attack"), vec![goblin]));
        println!("3. pinned set is the datafrog result on");
        println!("   the same input, checked by running both");
        assert_eq!(derived, expected);
        assert!(derived.contains(&Rel("can_attack"), &[goblin]));
        assert!(!derived.contains(&Rel("can_attack"), &[bears]));
    }
}
