use crate::Atom;
use crate::Case;
use crate::Facts;
use crate::Literal;
use crate::Program;
use crate::Rel;
use crate::Rule;
use crate::Term;

pub fn count_unstratifiable() -> Case {
    let p = Rel("p");
    Case {
        name: "count_unstratifiable",
        program: Program {
            rules: vec![Rule {
                head: Atom {
                    rel: p,
                    terms: vec![Term::Var("X")],
                },
                body: vec![Literal::Count {
                    output: "N",
                    body: vec![Atom {
                        rel: p,
                        terms: vec![Term::Var("X")],
                    }],
                }],
            }],
        },
        facts: Facts::default(),
        expected: Facts::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::count_unstratifiable;

    #[test]
    fn test_usage() {
        let case = count_unstratifiable();
        assert_eq!(case.name, "count_unstratifiable");
        assert!(case.program.stratify().is_err());
    }
}
