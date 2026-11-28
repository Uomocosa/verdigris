use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::EngineError;
use crate::Literal;
use crate::Program;
use crate::Rel;
use crate::Rule;
use crate::Term;

pub fn stratify(program: &Program) -> Result<Vec<Vec<Rule>>, EngineError> {
    let mut ordered: Vec<Rule> = program.rules.clone();
    ordered.sort();
    for rule in &ordered {
        let mut bound: BTreeSet<&'static str> = BTreeSet::new();
        for literal in &rule.body {
            if let Literal::Pos(atom) = literal {
                for term in &atom.terms {
                    if let Term::Var(name) = term {
                        bound.insert(*name);
                    }
                }
            }
        }
        let mut checked: Vec<&Term> = Vec::new();
        for term in &rule.head.terms {
            checked.push(term);
        }
        for literal in &rule.body {
            match literal {
                Literal::Pos(_) => {}
                Literal::Neg(atom) => {
                    for term in &atom.terms {
                        checked.push(term);
                    }
                }
                Literal::Cmp(left, _, right) => {
                    checked.push(left);
                    checked.push(right);
                }
            }
        }
        for term in checked {
            if let Term::Var(name) = term
                && !bound.contains(name)
            {
                return Err(EngineError::Unsafe {
                    relation: rule.head.rel,
                    variable: name,
                });
            }
        }
    }
    let mut strata: BTreeMap<Rel, u64> = BTreeMap::new();
    for rule in &ordered {
        strata.entry(rule.head.rel).or_insert(0);
        for literal in &rule.body {
            match literal {
                Literal::Pos(atom) | Literal::Neg(atom) => {
                    strata.entry(atom.rel).or_insert(0);
                }
                Literal::Cmp(..) => {}
            }
        }
    }
    let required = |rule: &Rule, strata: &BTreeMap<Rel, u64>| -> u64 {
        let mut level: u64 = strata.get(&rule.head.rel).copied().unwrap_or(0);
        for literal in &rule.body {
            match literal {
                Literal::Pos(atom) => {
                    if let Some(dependency) = strata.get(&atom.rel)
                        && *dependency > level
                    {
                        level = *dependency;
                    }
                }
                Literal::Neg(atom) => {
                    let dependency = strata.get(&atom.rel).copied().unwrap_or(0).saturating_add(1);
                    if dependency > level {
                        level = dependency;
                    }
                }
                Literal::Cmp(..) => {}
            }
        }
        level
    };
    let relation_count = strata.len();
    for _ in 0..relation_count {
        for rule in &ordered {
            let level = required(rule, &strata);
            strata.insert(rule.head.rel, level);
        }
    }
    for rule in &ordered {
        let level = required(rule, &strata);
        let current = strata.get(&rule.head.rel).copied().unwrap_or(0);
        if level > current {
            return Err(EngineError::Unstratifiable {
                relation: rule.head.rel,
            });
        }
    }
    let mut grouped: BTreeMap<u64, Vec<Rule>> = BTreeMap::new();
    for rule in ordered {
        let level = strata.get(&rule.head.rel).copied().unwrap_or(0);
        grouped.entry(level).or_default().push(rule);
    }
    Ok(grouped.into_values().collect())
}

#[cfg(test)]
mod tests {
    use crate::Atom;
    use crate::EngineError;
    use crate::Literal;
    use crate::Program;
    use crate::Rel;
    use crate::Rule;
    use crate::Term;
    use crate::Value;

    use super::stratify;

    #[test]
    fn test_usage() {
        let ancestor = Rule {
            head: Atom {
                rel: Rel("ancestor"),
                terms: vec![Term::Var("X"), Term::Var("Y")],
            },
            body: vec![Literal::Pos(Atom {
                rel: Rel("parent"),
                terms: vec![Term::Var("X"), Term::Var("Y")],
            })],
        };
        let program = Program {
            rules: vec![ancestor.clone()],
        };
        assert_eq!(stratify(&program), Ok(vec![vec![ancestor]]));
    }

    #[test]
    fn negative_dependency_stratum_runs_before_its_user() {
        let uses_negation = Rule {
            head: Atom {
                rel: Rel("p"),
                terms: vec![Term::Var("X")],
            },
            body: vec![
                Literal::Pos(Atom {
                    rel: Rel("q"),
                    terms: vec![Term::Var("X")],
                }),
                Literal::Neg(Atom {
                    rel: Rel("r"),
                    terms: vec![Term::Var("X")],
                }),
            ],
        };
        let defines_negation = Rule {
            head: Atom {
                rel: Rel("r"),
                terms: vec![Term::Var("X")],
            },
            body: vec![Literal::Pos(Atom {
                rel: Rel("s"),
                terms: vec![Term::Var("X")],
            })],
        };
        let program = Program {
            rules: vec![uses_negation.clone(), defines_negation.clone()],
        };
        assert_eq!(
            stratify(&program),
            Ok(vec![vec![defines_negation], vec![uses_negation]])
        );
    }

    #[test]
    fn cycle_through_negation_is_unstratifiable() {
        let program = Program {
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
        assert_eq!(
            stratify(&program),
            Err(EngineError::Unstratifiable { relation: Rel("p") })
        );
    }

    #[test]
    fn variable_only_in_negation_is_unsafe() {
        let program = Program {
            rules: vec![Rule {
                head: Atom {
                    rel: Rel("p"),
                    terms: vec![Term::Var("X")],
                },
                body: vec![Literal::Neg(Atom {
                    rel: Rel("q"),
                    terms: vec![Term::Var("X")],
                })],
            }],
        };
        assert_eq!(
            stratify(&program),
            Err(EngineError::Unsafe {
                relation: Rel("p"),
                variable: "X",
            })
        );
    }

    #[test]
    fn grounded_comparison_needs_no_binding() {
        let program = Program {
            rules: vec![Rule {
                head: Atom {
                    rel: Rel("p"),
                    terms: vec![Term::Val(Value::Int(1))],
                },
                body: vec![Literal::Cmp(
                    Term::Val(Value::Int(1)),
                    crate::CmpOp::Lt,
                    Term::Val(Value::Int(2)),
                )],
            }],
        };
        assert!(stratify(&program).is_ok());
    }
}
