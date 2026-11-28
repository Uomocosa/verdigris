use crate::Atom;
use crate::Case;
use crate::CmpOp;
use crate::Facts;
use crate::Literal;
use crate::Program;
use crate::Rel;
use crate::Rule;
use crate::Term;
use crate::Value;

pub fn comparison_literal() -> Case {
    let num = Rel("num");
    let big = Rel("big");
    let dead = Rel("dead");
    let mut facts = Facts::default();
    for value in [1, 2, 3, 4, 5] {
        let _ = facts.add(num, vec![Value::Int(value)]);
    }
    let mut expected = Facts::default();
    for value in [1, 2, 3, 4, 5] {
        let _ = expected.add(num, vec![Value::Int(value)]);
    }
    let _ = expected.add(big, vec![Value::Int(4)]);
    let _ = expected.add(big, vec![Value::Int(5)]);
    Case {
        name: "comparison_literal",
        program: Program {
            rules: vec![
                Rule {
                    head: Atom {
                        rel: big,
                        terms: vec![Term::Var("X")],
                    },
                    body: vec![
                        Literal::Pos(Atom {
                            rel: num,
                            terms: vec![Term::Var("X")],
                        }),
                        Literal::Cmp(Term::Var("X"), CmpOp::Gt, Term::Val(Value::Int(3))),
                    ],
                },
                Rule {
                    head: Atom {
                        rel: dead,
                        terms: vec![Term::Val(Value::Int(1))],
                    },
                    body: vec![Literal::Cmp(
                        Term::Val(Value::Int(1)),
                        CmpOp::Gt,
                        Term::Val(Value::Int(2)),
                    )],
                },
            ],
        },
        facts,
        expected,
    }
}

#[cfg(test)]
mod tests {
    use super::comparison_literal;

    #[test]
    fn test_usage() {
        let case = comparison_literal();
        assert_eq!(case.name, "comparison_literal");
        assert_eq!(case.program.rules.len(), 2);
    }
}
