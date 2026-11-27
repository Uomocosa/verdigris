use std::collections::BTreeSet;

use verdigris_datalog_ir::CmpOp;
use verdigris_datalog_ir::Literal;
use verdigris_datalog_ir::Rule;
use verdigris_datalog_ir::Term;

use crate::CompiledRule;

pub fn from_rule(rule: &Rule) -> CompiledRule {
    let mut ordered: BTreeSet<&'static str> = BTreeSet::new();
    for literal in &rule.body {
        if let Literal::Pos(atom) = literal {
            for term in &atom.terms {
                if let Term::Var(name) = term {
                    ordered.insert(*name);
                }
            }
        }
    }
    let variables: Vec<&'static str> = ordered.into_iter().collect();
    let mut positions = std::collections::BTreeMap::new();
    for (index, name) in variables.iter().enumerate() {
        positions.insert(*name, index);
    }
    let mut positives = Vec::new();
    let mut negatives = Vec::new();
    let mut comparisons = Vec::new();
    let mut dead = false;
    for literal in &rule.body {
        match literal {
            Literal::Pos(atom) => positives.push(atom.clone()),
            Literal::Neg(atom) => negatives.push(atom.clone()),
            Literal::Cmp(left, op, right) => match (left, right) {
                (Term::Val(first), Term::Val(second)) => {
                    let holds = match op {
                        CmpOp::Eq => first == second,
                        CmpOp::Ne => first != second,
                        CmpOp::Lt => first < second,
                        CmpOp::Le => first <= second,
                        CmpOp::Gt => first > second,
                        CmpOp::Ge => first >= second,
                    };
                    if !holds {
                        dead = true;
                    }
                }
                _ => comparisons.push((*left, *op, *right)),
            },
        }
    }
    CompiledRule {
        variables,
        positions,
        positives,
        negatives,
        comparisons,
        head: rule.head.clone(),
        dead,
    }
}

#[cfg(test)]
mod tests {
    use verdigris_datalog_ir::Atom;
    use verdigris_datalog_ir::CmpOp;
    use verdigris_datalog_ir::Literal;
    use verdigris_datalog_ir::Rel;
    use verdigris_datalog_ir::Rule;
    use verdigris_datalog_ir::Term;
    use verdigris_datalog_ir::Value;

    use super::from_rule;

    #[test]
    fn test_usage() {
        let compiled = from_rule(&Rule {
            head: Atom {
                rel: Rel("big"),
                terms: vec![Term::Var("X")],
            },
            body: vec![
                Literal::Pos(Atom {
                    rel: Rel("num"),
                    terms: vec![Term::Var("X")],
                }),
                Literal::Cmp(Term::Var("X"), CmpOp::Gt, Term::Val(Value::Int(3))),
            ],
        });
        assert_eq!(compiled.variables, vec!["X"]);
        assert_eq!(compiled.positives.len(), 1);
        assert_eq!(compiled.comparisons.len(), 1);
        assert!(!compiled.dead);
    }

    #[test]
    fn grounded_false_comparison_kills_the_rule() {
        let compiled = from_rule(&Rule {
            head: Atom {
                rel: Rel("dead"),
                terms: vec![Term::Val(Value::Int(1))],
            },
            body: vec![Literal::Cmp(
                Term::Val(Value::Int(1)),
                CmpOp::Gt,
                Term::Val(Value::Int(2)),
            )],
        });
        assert!(compiled.dead);
        assert_eq!(compiled.comparisons, Vec::new());
    }
}
