use std::collections::BTreeSet;

use verdigris_datalog_ir::CmpOp;
use verdigris_datalog_ir::Facts;
use verdigris_datalog_ir::Literal;
use verdigris_datalog_ir::Program;
use verdigris_datalog_ir::Rel;
use verdigris_datalog_ir::Term;
use verdigris_datalog_ir::Value;

pub fn render_program_text(program: &Program, facts: &Facts) -> String {
    let render_value = |value: &Value| -> String {
        match value {
            Value::Int(number) => number.to_string(),
            Value::Entity(number) => number.to_string(),
            Value::Sym(text) => format!("\"{}\"", text.replace('\\', "\\\\").replace('"', "\\\"")),
        }
    };
    let render_term = |term: &Term| -> String {
        match term {
            Term::Var(name) => format!("?{name}"),
            Term::Val(value) => render_value(value),
        }
    };
    let render_atom = |relation: &Rel, terms: &[Term]| -> String {
        let mut text = relation.to_string();
        text.push('(');
        let mut first = true;
        for term in terms {
            if first {
                first = false;
            } else {
                text.push_str(", ");
            }
            text.push_str(&render_term(term));
        }
        text.push(')');
        text
    };
    let comparison_holds = |first: &Value, operation: CmpOp, second: &Value| -> bool {
        let ordering = first.cmp(second);
        match operation {
            CmpOp::Eq => ordering.is_eq(),
            CmpOp::Ne => !ordering.is_eq(),
            CmpOp::Lt => ordering.is_lt(),
            CmpOp::Le => ordering.is_le(),
            CmpOp::Gt => ordering.is_gt(),
            CmpOp::Ge => ordering.is_ge(),
        }
    };
    let comparison_text = |operation: CmpOp| -> &'static str {
        match operation {
            CmpOp::Eq => "=",
            CmpOp::Ne => "!=",
            CmpOp::Lt => "<",
            CmpOp::Le => "<=",
            CmpOp::Gt => ">",
            CmpOp::Ge => ">=",
        }
    };
    let mut text = String::new();
    for (relation, tuple) in facts.iter() {
        let terms: Vec<Term> = tuple.iter().map(|value| Term::Val(*value)).collect();
        text.push_str(&render_atom(&relation, &terms));
        text.push_str(".\n");
    }
    let mut heads: BTreeSet<Rel> = BTreeSet::new();
    for rule in &program.rules {
        let mut defeated = false;
        let mut conditions: Vec<String> = Vec::new();
        for literal in &rule.body {
            match literal {
                Literal::Pos(atom) => conditions.push(render_atom(&atom.rel, &atom.terms)),
                Literal::Neg(atom) => {
                    conditions.push(format!("~{}", render_atom(&atom.rel, &atom.terms)));
                }
                Literal::Cmp(left, operation, right) => {
                    if let (Term::Val(first), Term::Val(second)) = (left, right) {
                        if comparison_holds(first, *operation, second) {
                            continue;
                        }
                        defeated = true;
                        break;
                    }
                    conditions.push(format!(
                        "{} {} {}",
                        render_term(left),
                        comparison_text(*operation),
                        render_term(right)
                    ));
                }
            }
        }
        if defeated {
            continue;
        }
        heads.insert(rule.head.rel);
        text.push_str(&render_atom(&rule.head.rel, &rule.head.terms));
        if !conditions.is_empty() {
            text.push_str(" :- ");
            text.push_str(&conditions.join(", "));
        }
        text.push_str(".\n");
    }
    for head in &heads {
        let name: &str = head;
        text.push_str("@export ");
        text.push_str(name);
        text.push_str(" :- csv{}.\n");
    }
    text
}

#[cfg(test)]
mod tests {
    use verdigris_datalog_ir::Atom;
    use verdigris_datalog_ir::CmpOp;
    use verdigris_datalog_ir::Facts;
    use verdigris_datalog_ir::Literal;
    use verdigris_datalog_ir::Program;
    use verdigris_datalog_ir::Rel;
    use verdigris_datalog_ir::Rule;
    use verdigris_datalog_ir::Term;
    use verdigris_datalog_ir::Value;

    use super::render_program_text;

    #[test]
    fn test_usage() {
        let program = Program {
            rules: vec![Rule {
                head: Atom {
                    rel: Rel("path"),
                    terms: vec![Term::Var("X"), Term::Var("Y")],
                },
                body: vec![Literal::Pos(Atom {
                    rel: Rel("edge"),
                    terms: vec![Term::Var("X"), Term::Var("Y")],
                })],
            }],
        };
        let mut facts = Facts::default();
        assert!(facts.add(Rel("edge"), vec![Value::Int(1), Value::Int(2)]));
        assert_eq!(
            render_program_text(&program, &facts),
            "edge(1, 2).\npath(?X, ?Y) :- edge(?X, ?Y).\n@export path :- csv{}.\n"
        );
    }

    #[test]
    fn renders_negation_symbols_and_ground_comparisons() {
        let program = Program {
            rules: vec![
                Rule {
                    head: Atom {
                        rel: Rel("present"),
                        terms: vec![Term::Var("X")],
                    },
                    body: vec![
                        Literal::Pos(Atom {
                            rel: Rel("query"),
                            terms: vec![Term::Var("X")],
                        }),
                        Literal::Neg(Atom {
                            rel: Rel("missing"),
                            terms: vec![Term::Var("X")],
                        }),
                        Literal::Cmp(Term::Var("X"), CmpOp::Gt, Term::Val(Value::Int(0))),
                        Literal::Cmp(
                            Term::Val(Value::Int(1)),
                            CmpOp::Lt,
                            Term::Val(Value::Int(2)),
                        ),
                    ],
                },
                Rule {
                    head: Atom {
                        rel: Rel("dead"),
                        terms: vec![Term::Val(Value::Int(1))],
                    },
                    body: vec![Literal::Cmp(
                        Term::Val(Value::Int(1)),
                        CmpOp::Gt,
                        Term::Val(Value::Int(2)),
                    )],
                },
                Rule {
                    head: Atom {
                        rel: Rel("named"),
                        terms: vec![Term::Val(Value::Sym("ada"))],
                    },
                    body: vec![],
                },
            ],
        };
        let text = render_program_text(&program, &Facts::default());
        assert!(text.contains("present(?X) :- query(?X), ~missing(?X), ?X > 0.\n"));
        assert!(!text.contains("dead"));
        assert!(text.contains("named(\"ada\").\n"));
    }
}
