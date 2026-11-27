use crate::Atom;
use crate::Case;
use crate::Facts;
use crate::Literal;
use crate::Program;
use crate::Rel;
use crate::Rule;
use crate::Term;
use crate::Value;

pub fn constants_in_head_and_body() -> Case {
    let edge = Rel("edge");
    let always = Rel("always");
    let from_one = Rel("from_one");
    let mut facts = Facts::default();
    let _ = facts.add(edge, vec![Value::Int(1), Value::Int(2)]);
    let _ = facts.add(edge, vec![Value::Int(2), Value::Int(3)]);
    let mut expected = Facts::default();
    let _ = expected.add(edge, vec![Value::Int(1), Value::Int(2)]);
    let _ = expected.add(edge, vec![Value::Int(2), Value::Int(3)]);
    let _ = expected.add(always, vec![Value::Int(0)]);
    let _ = expected.add(from_one, vec![Value::Int(2)]);
    Case {
        name: "constants_in_head_and_body",
        program: Program {
            rules: vec![
                Rule {
                    head: Atom {
                        rel: always,
                        terms: vec![Term::Val(Value::Int(0))],
                    },
                    body: vec![],
                },
                Rule {
                    head: Atom {
                        rel: from_one,
                        terms: vec![Term::Var("X")],
                    },
                    body: vec![Literal::Pos(Atom {
                        rel: edge,
                        terms: vec![Term::Val(Value::Int(1)), Term::Var("X")],
                    })],
                },
            ],
        },
        facts,
        expected,
    }
}

#[cfg(test)]
mod tests {
    use super::constants_in_head_and_body;

    #[test]
    fn test_usage() {
        let case = constants_in_head_and_body();
        assert_eq!(case.name, "constants_in_head_and_body");
        assert_eq!(case.program.rules.len(), 2);
    }
}
