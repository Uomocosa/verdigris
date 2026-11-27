use crate::Atom;
use crate::Case;
use crate::Facts;
use crate::Literal;
use crate::Program;
use crate::Rel;
use crate::Rule;
use crate::Term;
use crate::Value;

pub fn two_independent_strata() -> Case {
    let base = Rel("base");
    let first = Rel("first");
    let other = Rel("other");
    let second = Rel("second");
    let mut expected = Facts::default();
    let _ = expected.add(base, vec![Value::Int(1)]);
    let _ = expected.add(first, vec![Value::Int(1)]);
    let _ = expected.add(other, vec![Value::Int(2)]);
    let _ = expected.add(second, vec![Value::Int(2)]);
    Case {
        name: "two_independent_strata",
        program: Program {
            rules: vec![
                Rule {
                    head: Atom {
                        rel: base,
                        terms: vec![Term::Val(Value::Int(1))],
                    },
                    body: vec![],
                },
                Rule {
                    head: Atom {
                        rel: first,
                        terms: vec![Term::Var("X")],
                    },
                    body: vec![Literal::Pos(Atom {
                        rel: base,
                        terms: vec![Term::Var("X")],
                    })],
                },
                Rule {
                    head: Atom {
                        rel: other,
                        terms: vec![Term::Val(Value::Int(2))],
                    },
                    body: vec![],
                },
                Rule {
                    head: Atom {
                        rel: second,
                        terms: vec![Term::Var("X")],
                    },
                    body: vec![Literal::Pos(Atom {
                        rel: other,
                        terms: vec![Term::Var("X")],
                    })],
                },
            ],
        },
        facts: Facts::default(),
        expected,
    }
}

#[cfg(test)]
mod tests {
    use super::two_independent_strata;

    #[test]
    fn test_usage() {
        let case = two_independent_strata();
        assert_eq!(case.name, "two_independent_strata");
        assert_eq!(case.program.rules.len(), 4);
    }
}
