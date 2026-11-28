use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::Atom;
use crate::EngineError;
use crate::Literal;
use crate::Program;
use crate::Rel;
use crate::Rule;
use crate::Term;

pub fn stratify(program: &Program) -> Result<Vec<Vec<Rule>>, EngineError> {
    let variable_name = |term: &Term| match term {
        Term::Var(name) => Some(*name),
        Term::Val(_) => None,
    };
    let bind_vars = |atoms: &[Atom], into: &mut BTreeSet<&'static str>| {
        for atom in atoms {
            into.extend(atom.terms.iter().filter_map(variable_name));
        }
    };
    let mut ordered: Vec<Rule> = program.rules.clone();
    ordered.sort();
    for rule in &ordered {
        let mut bound: BTreeSet<&'static str> = BTreeSet::new();
        let mut checked: BTreeSet<&'static str> = BTreeSet::new();
        checked.extend(rule.head.terms.iter().filter_map(variable_name));
        let mut has_count = false;
        for literal in &rule.body {
            match literal {
                Literal::Pos(atom) => bind_vars(std::slice::from_ref(atom), &mut bound),
                Literal::Neg(atom) => bind_vars(std::slice::from_ref(atom), &mut checked),
                Literal::Cmp(left, _, right) => {
                    checked.extend([*left, *right].iter().filter_map(variable_name));
                }
                Literal::Count { output, .. } => {
                    if has_count {
                        return Err(EngineError::Unsafe {
                            relation: rule.head.rel,
                            variable: output,
                        });
                    }
                    has_count = true;
                    bound.insert(output);
                    bound.extend(Program::count_group_variables(rule));
                }
            }
        }
        for name in checked {
            if !bound.contains(name) {
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
            let atoms: Vec<&Atom> = match literal {
                Literal::Pos(atom) | Literal::Neg(atom) => vec![atom],
                Literal::Count { body, .. } => body.iter().collect(),
                Literal::Cmp(..) => Vec::new(),
            };
            for atom in atoms {
                strata.entry(atom.rel).or_insert(0);
            }
        }
    }
    let required = |rule: &Rule, strata: &BTreeMap<Rel, u64>| -> u64 {
        let mut level: u64 = strata.get(&rule.head.rel).copied().unwrap_or(0);
        for literal in &rule.body {
            let (atoms, offset): (&[Atom], u64) = match literal {
                Literal::Pos(atom) => (std::slice::from_ref(atom), 0),
                Literal::Neg(atom) => (std::slice::from_ref(atom), 1),
                Literal::Count { body, .. } => (body.as_slice(), 1),
                Literal::Cmp(..) => (&[], 0),
            };
            for atom in atoms {
                let dependency = strata.get(&atom.rel).copied().unwrap_or(0).saturating_add(offset);
                if dependency > level {
                    level = dependency;
                }
            }
        }
        level
    };
    let relation_count = strata.len();
    for _ in 0..relation_count {
        for rule in &ordered {
            strata.insert(rule.head.rel, required(rule, &strata));
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
