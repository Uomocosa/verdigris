use crate::EngineError;
use crate::Facts;
use crate::Program;
use crate::traits::engine::Engine;
use atomic_delegate_macros::atomic_delegates;

#[derive(Debug, Clone, Copy)]
pub struct DatafrogEngine;

#[rustfmt::skip]
impl Engine for DatafrogEngine {
    fn evaluate(&self, program: &Program, facts: &Facts) -> Result<Facts, EngineError> { Self::evaluate_program(program, facts) }
}

#[atomic_delegates]
impl DatafrogEngine {
    pub(crate) fn evaluate_program(program: &Program, facts: &Facts) -> Result<Facts, EngineError> {
    }
}

#[cfg(test)]
mod tests {
    use crate::Facts;
    use crate::Program;
    use crate::traits::engine::Engine;

    use super::DatafrogEngine;

    #[test]
    fn test_usage() {
        let engine = DatafrogEngine;
        let derived = engine.evaluate(&Program::default(), &Facts::default()).unwrap();
        assert_eq!(derived, Facts::default());
    }

    #[test]
    fn test_examples_attack() {
        use crate::Atom;
        use crate::Literal;
        use crate::Rel;
        use crate::Rule;
        use crate::Term;
        use crate::Value;
        use crate::methods::datafrog_engine::evaluate_program;
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
            let derived = evaluate_program(&sub, &known).unwrap();
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
