use crate::Atom;
use crate::Case;
use crate::Facts;
use crate::Literal;
use crate::Program;
use crate::Rel;
use crate::Rule;
use crate::Term;
use crate::Value;

pub fn unstratifiable() -> Case {
    let query = Rel("query");
    let path = Rel("path");
    let mut facts = Facts::default();
    let _ = facts.add(query, vec![Value::Int(1)]);
    Case {
        name: "unstratifiable",
        program: Program {
            rules: vec![Rule {
                head: Atom {
                    rel: path,
                    terms: vec![Term::Var("X")],
                },
                body: vec![
                    Literal::Pos(Atom {
                        rel: query,
                        terms: vec![Term::Var("X")],
                    }),
                    Literal::Neg(Atom {
                        rel: path,
                        terms: vec![Term::Var("X")],
                    }),
                ],
            }],
        },
        facts,
        expected: Facts::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::unstratifiable;

    #[test]
    fn test_usage() {
        let case = unstratifiable();
        assert_eq!(case.name, "unstratifiable");
        assert!(case.program.stratify().is_err());
    }
}
