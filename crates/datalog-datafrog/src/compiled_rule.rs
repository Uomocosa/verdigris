use std::collections::BTreeMap;

use atomic_delegate_macros::atomic_delegates;
use datafrog::Relation;
use verdigris_datalog_ir::Atom;
use verdigris_datalog_ir::CmpOp;
use verdigris_datalog_ir::Rel;
use verdigris_datalog_ir::Rule;
use verdigris_datalog_ir::Term;
use verdigris_datalog_ir::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledRule {
    pub(crate) variables: Vec<&'static str>,
    pub(crate) positions: BTreeMap<&'static str, usize>,
    pub(crate) positives: Vec<Atom>,
    pub(crate) negatives: Vec<Atom>,
    pub(crate) comparisons: Vec<(Term, CmpOp, Term)>,
    pub(crate) head: Atom,
    pub(crate) dead: bool,
}

#[atomic_delegates]
impl From<&Rule> for CompiledRule {
    fn from(rule: &Rule) -> Self {}
}

impl CompiledRule {
    #[must_use]
    pub(crate) fn substitute(&self, term: &Term, bindings: &[Option<Value>]) -> Option<Value> {
        match term {
            Term::Val(value) => Some(*value),
            Term::Var(name) => {
                self.positions.get(name).and_then(|index| bindings.get(*index)).copied().flatten()
            }
        }
    }
}

#[rustfmt::skip]
impl CompiledRule {
    #[must_use]
    pub(crate) fn substitute_all(&self, terms: &[Term], bindings: &[Option<Value>]) -> Vec<Value> {
        terms.iter().map(|term| self.substitute(term, bindings).unwrap_or(Value::Int(0))).collect()
    }
}

#[atomic_delegates]
impl CompiledRule {
    #[must_use]
    pub(crate) fn comparison_holds(
        &self,
        comparison: &(Term, CmpOp, Term),
        bindings: &[Option<Value>],
    ) -> bool {
    }
    pub(crate) fn fact_seed(&self, negated: &[(Atom, Relation<Vec<Value>>)]) -> Option<Vec<Value>> {
    }
    pub(crate) fn ground_negations(
        &self,
        completed: &BTreeMap<Rel, Relation<Vec<Value>>>,
    ) -> Vec<(Atom, Relation<Vec<Value>>)> {
    }
}

#[cfg(test)]
mod tests {
    use verdigris_datalog_ir::Atom;
    use verdigris_datalog_ir::Literal;
    use verdigris_datalog_ir::Rel;
    use verdigris_datalog_ir::Rule;
    use verdigris_datalog_ir::Term;
    use verdigris_datalog_ir::Value;

    use super::CompiledRule;

    #[test]
    fn test_usage() {
        let rule = Rule {
            head: Atom {
                rel: Rel("path"),
                terms: vec![Term::Var("X"), Term::Var("Y")],
            },
            body: vec![],
        };
        let compiled = CompiledRule::from(&rule);
        assert_eq!(compiled.variables, Vec::<&str>::new());
        assert_eq!(
            compiled.substitute_all(&rule.head.terms, &[]),
            vec![Value::Int(0), Value::Int(0)]
        );
    }

    #[test]
    fn substitute_binds_positional_variables() {
        let compiled = CompiledRule::from(&Rule {
            head: Atom {
                rel: Rel("pair"),
                terms: vec![Term::Var("X"), Term::Val(Value::Int(9))],
            },
            body: vec![Literal::Pos(Atom {
                rel: Rel("source"),
                terms: vec![Term::Var("X")],
            })],
        });
        assert_eq!(
            compiled.substitute(&Term::Val(Value::Int(7)), &[]),
            Some(Value::Int(7))
        );
        assert_eq!(compiled.substitute(&Term::Var("X"), &[]), None);
    }

    #[test]
    fn substitute_all_projects_head_terms() {
        let rule = Rule {
            head: Atom {
                rel: Rel("pair"),
                terms: vec![Term::Var("X"), Term::Val(Value::Int(9))],
            },
            body: vec![Literal::Pos(Atom {
                rel: Rel("source"),
                terms: vec![Term::Var("X")],
            })],
        };
        let compiled = CompiledRule::from(&rule);
        assert_eq!(
            compiled.substitute_all(&rule.head.terms, &[Some(Value::Int(4))]),
            vec![Value::Int(4), Value::Int(9)]
        );
        assert_eq!(
            compiled.substitute_all(&rule.head.terms, &[]),
            vec![Value::Int(0), Value::Int(9)]
        );
    }

    #[test]
    fn test_examples_compiled_plan() {
        use verdigris_datalog_ir::CmpOp;
        let rule = Rule {
            head: Atom {
                rel: Rel("violates"),
                terms: vec![Term::Var("C"), Term::Val(Value::Sym("sick"))],
            },
            body: vec![
                Literal::Pos(Atom {
                    rel: Rel("creature"),
                    terms: vec![Term::Var("C")],
                }),
                Literal::Pos(Atom {
                    rel: Rel("sick"),
                    terms: vec![Term::Var("C")],
                }),
                Literal::Neg(Atom {
                    rel: Rel("waived"),
                    terms: vec![Term::Var("C"), Term::Val(Value::Sym("sick"))],
                }),
                Literal::Cmp(Term::Var("C"), CmpOp::Ne, Term::Val(Value::Sym("none"))),
            ],
        };
        let compiled = CompiledRule::from(&rule);
        println!("1. rule: {rule:?}");
        println!("2. variables in join order: {:?}", compiled.variables);
        println!("   positions: {:?}", compiled.positions);
        println!("3. join steps in order:");
        for (index, atom) in compiled.positives.iter().enumerate() {
            println!("   step {index}: join {:?} on {:?}", *atom.rel, atom.terms);
        }
        for atom in &compiled.negatives {
            println!("   antijoin: skip rows matching {atom:?}");
        }
        for (left, op, right) in &compiled.comparisons {
            println!("   filter: {left:?} {op:?} {right:?}");
        }
        assert_eq!(compiled.variables, vec!["C"]);
        assert_eq!(compiled.positives.len(), 2);
        assert_eq!(compiled.negatives.len(), 1);
        assert_eq!(compiled.comparisons.len(), 1);
        assert!(!compiled.dead);
    }
}
