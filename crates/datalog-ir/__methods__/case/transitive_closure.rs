use crate::Atom;
use crate::Case;
use crate::Facts;
use crate::Literal;
use crate::Program;
use crate::Rel;
use crate::Rule;
use crate::Term;
use crate::Value;

pub fn transitive_closure() -> Case {
    let edge = Rel("edge");
    let path = Rel("path");
    let mut facts = Facts::default();
    let _ = facts.add(edge, vec![Value::Int(1), Value::Int(2)]);
    let _ = facts.add(edge, vec![Value::Int(2), Value::Int(3)]);
    let _ = facts.add(edge, vec![Value::Int(3), Value::Int(4)]);
    let mut expected = Facts::default();
    for (relation, tuple) in [
        (edge, (1, 2)),
        (edge, (2, 3)),
        (edge, (3, 4)),
        (path, (1, 2)),
        (path, (1, 3)),
        (path, (1, 4)),
        (path, (2, 3)),
        (path, (2, 4)),
        (path, (3, 4)),
    ] {
        let _ = expected.add(relation, vec![Value::Int(tuple.0), Value::Int(tuple.1)]);
    }
    Case {
        name: "transitive_closure",
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
            ],
        },
        facts,
        expected,
    }
}

#[cfg(test)]
mod tests {
    use super::transitive_closure;

    #[test]
    fn test_usage() {
        let case = transitive_closure();
        assert_eq!(case.name, "transitive_closure");
        assert_eq!(case.program.rules.len(), 2);
    }
}
