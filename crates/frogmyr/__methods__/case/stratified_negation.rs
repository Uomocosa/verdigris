use crate::Atom;
use crate::Case;
use crate::Facts;
use crate::Literal;
use crate::Program;
use crate::Rel;
use crate::Rule;
use crate::Term;
use crate::Value;

pub fn stratified_negation() -> Case {
    let edge = Rel("edge");
    let node = Rel("node");
    let reachable = Rel("reachable");
    let unreachable = Rel("unreachable");
    let mut facts = Facts::default();
    let _ = facts.add(node, vec![Value::Int(1)]);
    let _ = facts.add(node, vec![Value::Int(2)]);
    let _ = facts.add(node, vec![Value::Int(3)]);
    let _ = facts.add(edge, vec![Value::Int(1), Value::Int(2)]);
    let mut expected = Facts::default();
    for (relation, tuple) in [
        (node, vec![Value::Int(1)]),
        (node, vec![Value::Int(2)]),
        (node, vec![Value::Int(3)]),
        (edge, vec![Value::Int(1), Value::Int(2)]),
        (reachable, vec![Value::Int(1)]),
        (reachable, vec![Value::Int(2)]),
        (unreachable, vec![Value::Int(3)]),
    ] {
        let _ = expected.add(relation, tuple);
    }
    Case {
        name: "stratified_negation",
        program: Program {
            rules: vec![
                Rule {
                    head: Atom {
                        rel: reachable,
                        terms: vec![Term::Val(Value::Int(1))],
                    },
                    body: vec![],
                },
                Rule {
                    head: Atom {
                        rel: reachable,
                        terms: vec![Term::Var("Y")],
                    },
                    body: vec![
                        Literal::Pos(Atom {
                            rel: reachable,
                            terms: vec![Term::Var("X")],
                        }),
                        Literal::Pos(Atom {
                            rel: edge,
                            terms: vec![Term::Var("X"), Term::Var("Y")],
                        }),
                    ],
                },
                Rule {
                    head: Atom {
                        rel: unreachable,
                        terms: vec![Term::Var("X")],
                    },
                    body: vec![
                        Literal::Pos(Atom {
                            rel: node,
                            terms: vec![Term::Var("X")],
                        }),
                        Literal::Neg(Atom {
                            rel: reachable,
                            terms: vec![Term::Var("X")],
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
    use super::stratified_negation;

    #[test]
    fn test_usage() {
        let case = stratified_negation();
        assert_eq!(case.name, "stratified_negation");
        assert_eq!(case.program.rules.len(), 3);
    }
}
