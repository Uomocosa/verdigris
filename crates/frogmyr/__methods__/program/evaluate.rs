use std::collections::BTreeMap;
use std::collections::BTreeSet;

use datafrog::Iteration;
use datafrog::Relation;
use datafrog::Variable;

use crate::COUNT_RELATION;
use crate::CompiledRule;
use crate::EngineError;
use crate::Facts;
use crate::Literal;
use crate::Program;
use crate::Rel;
use crate::Value;
use crate::WiredRule;

pub fn evaluate(program: &Program, facts: &Facts) -> Result<Facts, EngineError> {
    let strata = program.stratify()?;
    let mut state: BTreeMap<Rel, BTreeSet<Vec<Value>>> = BTreeMap::new();
    for (relation, tuple) in facts.iter() {
        state.entry(relation).or_default().insert(tuple.to_vec());
    }
    for stratum in &strata {
        let mut heads: BTreeSet<Rel> = BTreeSet::new();
        for rule in stratum {
            heads.insert(rule.head.rel);
        }
        let mut completed: BTreeMap<Rel, Relation<Vec<Value>>> = BTreeMap::new();
        for (relation, known) in &state {
            if heads.contains(relation) {
                continue;
            }
            let mut tuples: Vec<Vec<Value>> = known.iter().cloned().collect();
            tuples.sort();
            tuples.dedup();
            completed.insert(*relation, Relation::from_vec(tuples));
        }
        let mut iteration = Iteration::new();
        let mut rows: BTreeMap<Rel, Variable<Vec<Value>>> = BTreeMap::new();
        for head in &heads {
            let variable = iteration.variable::<Vec<Value>>("rows");
            if let Some(known) = state.get(head) {
                variable.extend(known.iter().cloned());
            }
            rows.insert(*head, variable);
        }
        let mut wirings = Vec::with_capacity(stratum.len());
        for rule in stratum {
            let plan = CompiledRule::from(rule);
            let Some(head_rows) = rows.get(&plan.head.rel) else {
                continue;
            };
            let aggregate =
                if rule.body.iter().any(|literal| matches!(literal, Literal::Count { .. })) {
                    let variable = iteration.variable::<Vec<Value>>("aggregate");
                    variable.extend(Program::count_rows(rule, &completed));
                    Some(variable)
                } else {
                    None
                };
            let mut sources = Vec::with_capacity(plan.positives.len());
            for atom in &plan.positives {
                if atom.rel == COUNT_RELATION {
                    let Some(source) = aggregate.as_ref() else {
                        continue;
                    };
                    sources.push(source.clone());
                } else if heads.contains(&atom.rel) {
                    let Some(source) = rows.get(&atom.rel) else {
                        continue;
                    };
                    sources.push(source.clone());
                } else {
                    let source = iteration.variable::<Vec<Value>>("source");
                    if let Some(relation) = completed.get(&atom.rel) {
                        source.extend(relation.iter().cloned());
                    }
                    sources.push(source);
                }
            }
            wirings.push(WiredRule::from_plan(
                plan,
                &mut iteration,
                head_rows,
                &sources,
                &completed,
            ));
        }
        while iteration.changed() {
            for wiring in &wirings {
                wiring.pump();
            }
        }
        for (head, variable) in rows {
            let derived = variable.complete();
            for tuple in derived.iter() {
                state.entry(head).or_default().insert(tuple.clone());
            }
        }
    }
    let mut out = Facts::default();
    for (relation, known) in &state {
        for tuple in known {
            let _ = out.add(*relation, tuple.clone());
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use crate::Atom;
    use crate::Case;
    use crate::Facts;
    use crate::Literal;
    use crate::Program;
    use crate::Rel;
    use crate::Rule;
    use crate::Term;
    use crate::Value;

    use super::evaluate;

    #[test]
    fn test_usage() {
        let derived = evaluate(&Program::default(), &Facts::default()).unwrap();
        assert_eq!(derived, Facts::default());
        assert_eq!(
            Program::default().evaluate(&Facts::default()).unwrap(),
            Facts::default()
        );
    }

    #[test]
    fn transitive_closure_derives_paths() {
        let edge = Rel("edge");
        let path = Rel("path");
        let program = Program {
            rules: vec![
                Rule {
                    head: Atom {
                        rel: path,
                        terms: vec![Term::Var("X"), Term::Var("Y")],
                    },
                    body: vec![Literal::Pos(Atom {
                        rel: edge,
                        terms: vec![Term::Var("X"), Term::Var("Y")],
                    })],
                },
                Rule {
                    head: Atom {
                        rel: path,
                        terms: vec![Term::Var("X"), Term::Var("Z")],
                    },
                    body: vec![
                        Literal::Pos(Atom {
                            rel: path,
                            terms: vec![Term::Var("X"), Term::Var("Y")],
                        }),
                        Literal::Pos(Atom {
                            rel: edge,
                            terms: vec![Term::Var("Y"), Term::Var("Z")],
                        }),
                    ],
                },
            ],
        };
        let mut facts = Facts::default();
        assert!(facts.add(edge, vec![Value::Int(1), Value::Int(2)]));
        assert!(facts.add(edge, vec![Value::Int(2), Value::Int(3)]));
        let derived = evaluate(&program, &facts).unwrap();
        assert!(derived.contains(&path, &[Value::Int(1), Value::Int(3)]));
        assert!(!derived.contains(&path, &[Value::Int(3), Value::Int(1)]));
    }

    #[test]
    fn shared_conformance_cases_pass() {
        let started = std::time::Instant::now();
        let outcome = Case::check();
        let elapsed = started.elapsed();
        println!(
            "shared conformance: {outcome:?} in {}ms",
            elapsed.as_millis()
        );
        assert!(outcome.is_ok());
        let mut chain: Facts = Facts::default();
        for end in 1..300 {
            assert!(chain.add(
                Rel("edge"),
                vec![Value::Int(end), Value::Int(end.saturating_add(1))]
            ));
        }
        let edge = Rel("edge");
        let path = Rel("path");
        let program = Program {
            rules: vec![
                Rule {
                    head: Atom {
                        rel: path,
                        terms: vec![Term::Var("X"), Term::Var("Y")],
                    },
                    body: vec![Literal::Pos(Atom {
                        rel: edge,
                        terms: vec![Term::Var("X"), Term::Var("Y")],
                    })],
                },
                Rule {
                    head: Atom {
                        rel: path,
                        terms: vec![Term::Var("X"), Term::Var("Z")],
                    },
                    body: vec![
                        Literal::Pos(Atom {
                            rel: path,
                            terms: vec![Term::Var("X"), Term::Var("Y")],
                        }),
                        Literal::Pos(Atom {
                            rel: edge,
                            terms: vec![Term::Var("Y"), Term::Var("Z")],
                        }),
                    ],
                },
            ],
        };
        let probe_started = std::time::Instant::now();
        let derived = evaluate(&program, &chain).unwrap();
        let probe_elapsed = probe_started.elapsed();
        println!(
            "chain of 300: {} path tuples in {}ms",
            derived.matching(&path, &[Term::Var("P"), Term::Var("Q")]).len(),
            probe_elapsed.as_millis()
        );
        assert!(derived.contains(&path, &[Value::Int(1), Value::Int(300)]));
    }

    #[test]
    fn test_examples_attack() {
        let (bears, goblin) = (Value::Sym("bears"), Value::Sym("goblin"));
        let mut facts = Facts::default();
        for name in [bears, goblin] {
            assert!(facts.add(Rel("creature"), vec![name]));
        }
        assert!(facts.add(Rel("tapped"), vec![bears]));
        assert!(facts.add(Rel("sick"), vec![goblin]));
        assert!(facts.add(Rel("has_haste"), vec![goblin]));
        println!("1. facts in:");
        for (relation, tuple) in facts.iter() {
            println!("   {}({tuple:?})", *relation);
        }
        let pos1 = |rel, var| {
            Literal::Pos(Atom {
                rel,
                terms: vec![Term::Var(var)],
            })
        };
        let tagged = |rel, var, tag| Atom {
            rel,
            terms: vec![Term::Var(var), Term::Val(Value::Sym(tag))],
        };
        let program = Program {
            rules: vec![
                Rule {
                    head: tagged(Rel("waived"), "G", "sick"),
                    body: vec![pos1(Rel("has_haste"), "G")],
                },
                Rule {
                    head: tagged(Rel("violates"), "C", "untapped"),
                    body: vec![pos1(Rel("creature"), "C"), pos1(Rel("tapped"), "C")],
                },
                Rule {
                    head: tagged(Rel("violates"), "C", "sick"),
                    body: vec![
                        pos1(Rel("creature"), "C"),
                        pos1(Rel("sick"), "C"),
                        Literal::Neg(tagged(Rel("waived"), "C", "sick")),
                    ],
                },
                Rule {
                    head: Atom {
                        rel: Rel("can_attack"),
                        terms: vec![Term::Var("C")],
                    },
                    body: vec![
                        pos1(Rel("creature"), "C"),
                        Literal::Neg(tagged(Rel("violates"), "C", "untapped")),
                        Literal::Neg(tagged(Rel("violates"), "C", "sick")),
                    ],
                },
            ],
        };
        let mut known = facts.clone();
        for (index, stratum) in program.stratify().unwrap().iter().enumerate() {
            println!("2. stratum {index}:");
            for rule in stratum {
                let mut waits: Vec<&str> = Vec::new();
                for literal in &rule.body {
                    if let Literal::Neg(atom) = literal {
                        waits.push(*atom.rel);
                    }
                }
                println!("   {:?} waits on {waits:?}", *rule.head.rel);
            }
            let sub = Program {
                rules: stratum.clone(),
            };
            let derived = evaluate(&sub, &known).unwrap();
            println!("3. stratum {index} new facts:");
            for (relation, tuple) in derived.iter() {
                if !known.contains(&relation, tuple) {
                    println!("   {}({tuple:?})", *relation);
                }
            }
            known = derived;
        }
        println!("4. verdict per creature:");
        for creature in [bears, goblin] {
            if known.contains(&Rel("can_attack"), &[creature]) {
                println!("   {creature:?} can attack");
            } else {
                println!("   {creature:?} cannot attack");
            }
        }
        let reasons = known.matching(&Rel("violates"), &[Term::Var("C"), Term::Var("R")]);
        println!("   why not: {reasons:?}");
        assert!(known.contains(&Rel("waived"), &[goblin, Value::Sym("sick")]));
        assert!(known.contains(&Rel("violates"), &[bears, Value::Sym("untapped")]));
        assert!(!known.contains(&Rel("violates"), &[goblin, Value::Sym("sick")]));
        assert!(known.contains(&Rel("can_attack"), &[goblin]));
        assert!(!known.contains(&Rel("can_attack"), &[bears]));
    }
}
