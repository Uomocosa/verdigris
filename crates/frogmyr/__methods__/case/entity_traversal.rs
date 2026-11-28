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

pub fn entity_traversal() -> Case {
    let edge = Rel("edge");
    let path = Rel("path");
    let score = Rel("score");
    let high = Rel("high");
    let mut facts = Facts::default();
    let _ = facts.add(edge, vec![Value::Entity(1), Value::Entity(2)]);
    let _ = facts.add(edge, vec![Value::Entity(2), Value::Entity(3)]);
    let _ = facts.add(edge, vec![Value::Entity(3), Value::Entity(4)]);
    let _ = facts.add(score, vec![Value::Entity(1), Value::Int(10)]);
    let _ = facts.add(score, vec![Value::Entity(2), Value::Int(30)]);
    let mut expected = Facts::default();
    for (first, second) in [(1, 2), (2, 3), (3, 4)] {
        let _ = expected.add(edge, vec![Value::Entity(first), Value::Entity(second)]);
    }
    for (first, second) in [(1, 2), (2, 3), (3, 4), (1, 3), (1, 4), (2, 4)] {
        let _ = expected.add(path, vec![Value::Entity(first), Value::Entity(second)]);
    }
    let _ = expected.add(score, vec![Value::Entity(1), Value::Int(10)]);
    let _ = expected.add(score, vec![Value::Entity(2), Value::Int(30)]);
    let _ = expected.add(high, vec![Value::Entity(2)]);
    Case {
        name: "entity_traversal",
        program: Program {
            rules: vec![
                Rule {
                    head: Atom {
                        rel: path,
                        terms: vec![Term::Var("X"), Term::Var("Y")],
                    },
                    body: vec![Literal::Pos(Atom {
                        rel: edge,
                        terms: vec![Term::Var("X"), Term::Var("Y")],
                    })],
                },
                Rule {
                    head: Atom {
                        rel: path,
                        terms: vec![Term::Var("X"), Term::Var("Z")],
                    },
                    body: vec![
                        Literal::Pos(Atom {
                            rel: path,
                            terms: vec![Term::Var("X"), Term::Var("Y")],
                        }),
                        Literal::Pos(Atom {
                            rel: edge,
                            terms: vec![Term::Var("Y"), Term::Var("Z")],
                        }),
                    ],
                },
                Rule {
                    head: Atom {
                        rel: high,
                        terms: vec![Term::Var("X")],
                    },
                    body: vec![
                        Literal::Pos(Atom {
                            rel: score,
                            terms: vec![Term::Var("X"), Term::Var("P")],
                        }),
                        Literal::Cmp(Term::Var("P"), CmpOp::Gt, Term::Val(Value::Int(20))),
                    ],
                },
            ],
        },
        facts,
        expected,
    }
}

#[cfg(test)]
mod tests {
    use super::entity_traversal;

    #[test]
    fn test_usage() {
        let case = entity_traversal();
        assert_eq!(case.name, "entity_traversal");
        assert_eq!(case.program.rules.len(), 3);
    }
}
