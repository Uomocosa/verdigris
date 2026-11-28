use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;

use crate::EngineError;
use crate::Facts;
use crate::Rule;

#[derive(Debug, Clone, Default, PartialEq, Eq, Deref)]
pub struct Program {
    pub rules: Vec<Rule>,
}

#[atomic_delegates]
impl Program {
    pub fn evaluate(&self, facts: &Facts) -> Result<Facts, EngineError> {}
    pub fn stratify(&self) -> Result<Vec<Vec<Rule>>, EngineError> {}
}

impl Program {
    #[must_use]
    pub fn canonical(mut self) -> Self {
        self.rules.sort();
        self.rules.dedup();
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::Atom;
    use crate::Rel;
    use crate::Rule;
    use crate::Term;
    use crate::Value;

    use super::Program;

    #[test]
    fn test_usage() {
        let later = Rule {
            head: Atom {
                rel: Rel("edge"),
                terms: vec![Term::Val(Value::Int(2)), Term::Val(Value::Int(3))],
            },
            body: vec![],
        };
        let earlier = Rule {
            head: Atom {
                rel: Rel("edge"),
                terms: vec![Term::Val(Value::Int(1)), Term::Val(Value::Int(2))],
            },
            body: vec![],
        };
        let canonical = Program {
            rules: vec![later.clone(), earlier.clone()],
        }
        .canonical();
        assert_eq!(canonical.rules, vec![earlier, later]);
    }

    #[test]
    fn canonical_is_order_independent() {
        let first = Rule {
            head: Atom {
                rel: Rel("edge"),
                terms: vec![Term::Val(Value::Int(1)), Term::Val(Value::Int(2))],
            },
            body: vec![],
        };
        let second = Rule {
            head: Atom {
                rel: Rel("edge"),
                terms: vec![Term::Val(Value::Int(2)), Term::Val(Value::Int(3))],
            },
            body: vec![],
        };
        let forward = Program {
            rules: vec![first.clone(), second.clone()],
        }
        .canonical();
        let backward = Program {
            rules: vec![second, first.clone(), first],
        }
        .canonical();
        assert_eq!(forward, backward);
    }

    #[test]
    fn test_examples_stratify() {
        use crate::Literal;
        let reachable = Rule {
            head: Atom {
                rel: Rel("reachable"),
                terms: vec![Term::Var("Y")],
            },
            body: vec![
                Literal::Pos(Atom {
                    rel: Rel("reachable"),
                    terms: vec![Term::Var("X")],
                }),
                Literal::Pos(Atom {
                    rel: Rel("edge"),
                    terms: vec![Term::Var("X"), Term::Var("Y")],
                }),
            ],
        };
        let blocked = Rule {
            head: Atom {
                rel: Rel("blocked"),
                terms: vec![Term::Var("X")],
            },
            body: vec![
                Literal::Pos(Atom {
                    rel: Rel("node"),
                    terms: vec![Term::Var("X")],
                }),
                Literal::Neg(Atom {
                    rel: Rel("reachable"),
                    terms: vec![Term::Var("X")],
                }),
            ],
        };
        let program = Program {
            rules: vec![blocked.clone(), reachable.clone()],
        };
        println!("1. rules in:");
        for rule in &program.rules {
            println!("   {rule:?}");
        }
        let strata = program.stratify().unwrap();
        println!("2. stratum 0 derives reachable from edge");
        println!("3. stratum 1 derives blocked after reachable");
        println!("   is complete, as blocked waits on not reachable");
        for (index, stratum) in strata.iter().enumerate() {
            println!("   stratum {index}: {} rule(s)", stratum.len());
        }
        assert_eq!(strata, vec![vec![reachable], vec![blocked]]);
        let bad = Program {
            rules: vec![Rule {
                head: Atom {
                    rel: Rel("p"),
                    terms: vec![],
                },
                body: vec![Literal::Neg(Atom {
                    rel: Rel("p"),
                    terms: vec![],
                })],
            }],
        };
        let error = bad.stratify().unwrap_err();
        println!("4. rejected: {error}");
        assert_eq!(
            error,
            crate::EngineError::Unstratifiable { relation: Rel("p") }
        );
    }
}
