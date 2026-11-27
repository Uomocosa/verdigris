use crate::Atom;
use crate::Case;
use crate::Facts;
use crate::Literal;
use crate::Program;
use crate::Rel;
use crate::Rule;
use crate::Term;
use crate::Value;

pub fn unsafe_head() -> Case {
    let query = Rel("query");
    let path = Rel("path");
    let mut facts = Facts::default();
    let _ = facts.add(query, vec![Value::Int(1)]);
    Case {
        name: "unsafe_head",
        program: Program {
            rules: vec![Rule {
                head: Atom {
                    rel: path,
                    terms: vec![Term::Var("X")],
                },
                body: vec![Literal::Pos(Atom {
                    rel: query,
                    terms: vec![Term::Var("Y")],
                })],
            }],
        },
        facts,
        expected: Facts::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::unsafe_head;

    #[test]
    fn test_usage() {
        let case = unsafe_head();
        assert_eq!(case.name, "unsafe_head");
        assert!(case.program.stratify().is_err());
    }
}
