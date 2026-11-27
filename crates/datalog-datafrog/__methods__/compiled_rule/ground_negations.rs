use std::collections::BTreeMap;

use datafrog::Relation;
use verdigris_datalog_ir::Atom;
use verdigris_datalog_ir::Rel;
use verdigris_datalog_ir::Value;

use crate::CompiledRule;

pub fn ground_negations(
    rule: &CompiledRule,
    completed: &BTreeMap<Rel, Relation<Vec<Value>>>,
) -> Vec<(Atom, Relation<Vec<Value>>)> {
    let mut negated = Vec::with_capacity(rule.negatives.len());
    for atom in &rule.negatives {
        let relation =
            completed.get(&atom.rel).cloned().unwrap_or_else(|| Relation::from_vec(Vec::new()));
        negated.push((atom.clone(), relation));
    }
    negated
}

#[cfg(test)]
mod tests {
    use verdigris_datalog_ir::Atom;
    use verdigris_datalog_ir::Literal;
    use verdigris_datalog_ir::Rel;
    use verdigris_datalog_ir::Rule;
    use verdigris_datalog_ir::Term;

    use super::ground_negations;
    use crate::CompiledRule;

    #[test]
    fn test_usage() {
        let rule = CompiledRule::from(&Rule {
            head: Atom {
                rel: Rel("present"),
                terms: vec![Term::Var("X")],
            },
            body: vec![
                Literal::Pos(Atom {
                    rel: Rel("query"),
                    terms: vec![Term::Var("X")],
                }),
                Literal::Neg(Atom {
                    rel: Rel("missing"),
                    terms: vec![Term::Var("X")],
                }),
            ],
        });
        let negated = ground_negations(&rule, &std::collections::BTreeMap::default());
        assert_eq!(negated.len(), 1);
        assert!(negated[0].1.is_empty());
    }
}
