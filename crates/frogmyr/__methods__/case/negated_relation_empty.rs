use crate::Atom;
use crate::Case;
use crate::Facts;
use crate::Literal;
use crate::Program;
use crate::Rel;
use crate::Rule;
use crate::Term;
use crate::Value;

pub fn negated_relation_empty() -> Case {
    let query = Rel("query");
    let missing = Rel("missing");
    let present = Rel("present");
    let mut facts = Facts::default();
    let _ = facts.add(query, vec![Value::Int(1)]);
    let _ = facts.add(query, vec![Value::Int(2)]);
    let mut expected = Facts::default();
    let _ = expected.add(query, vec![Value::Int(1)]);
    let _ = expected.add(query, vec![Value::Int(2)]);
    let _ = expected.add(present, vec![Value::Int(1)]);
    let _ = expected.add(present, vec![Value::Int(2)]);
    Case {
        name: "negated_relation_empty",
        program: Program {
            rules: vec![Rule {
                head: Atom {
                    rel: present,
                    terms: vec![Term::Var("X")],
                },
                body: vec![
                    Literal::Pos(Atom {
                        rel: query,
                        terms: vec![Term::Var("X")],
                    }),
                    Literal::Neg(Atom {
                        rel: missing,
                        terms: vec![Term::Var("X")],
                    }),
                ],
            }],
        },
        facts,
        expected,
    }
}

#[cfg(test)]
mod tests {
    use super::negated_relation_empty;

    #[test]
    fn test_usage() {
        let case = negated_relation_empty();
        assert_eq!(case.name, "negated_relation_empty");
        assert_eq!(case.program.rules.len(), 1);
    }
}
