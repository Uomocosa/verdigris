use crate::Atom;
use crate::Value;
use datafrog::Relation;

use crate::CompiledRule;

pub fn fact_seed(
    rule: &CompiledRule,
    negated: &[(Atom, Relation<Vec<Value>>)],
) -> Option<Vec<Value>> {
    if !rule.positives.is_empty() || rule.dead {
        return None;
    }
    for (atom, relation) in negated {
        let key = rule.substitute_all(&atom.terms, &[]);
        if relation.iter().any(|tuple| tuple == &key) {
            return None;
        }
    }
    Some(rule.substitute_all(&rule.head.terms, &[]))
}

#[cfg(test)]
mod tests {
    use crate::Atom;
    use crate::Literal;
    use crate::Rel;
    use crate::Rule;
    use crate::Term;
    use crate::Value;

    use super::fact_seed;
    use crate::CompiledRule;

    #[test]
    fn test_usage() {
        let rule = CompiledRule::from(&Rule {
            head: Atom {
                rel: Rel("always"),
                terms: vec![Term::Val(Value::Int(0))],
            },
            body: vec![],
        });
        assert_eq!(fact_seed(&rule, &[]), Some(vec![Value::Int(0)]));
    }

    #[test]
    fn blocked_when_a_ground_negation_holds() {
        let rule = CompiledRule::from(&Rule {
            head: Atom {
                rel: Rel("maybe"),
                terms: vec![Term::Val(Value::Int(1))],
            },
            body: vec![Literal::Neg(Atom {
                rel: Rel("blocked"),
                terms: vec![Term::Val(Value::Int(1))],
            })],
        });
        let present: datafrog::Relation<Vec<Value>> =
            datafrog::Relation::from_vec(vec![vec![Value::Int(1)]]);
        let negated = vec![(
            Atom {
                rel: Rel("blocked"),
                terms: vec![Term::Val(Value::Int(1))],
            },
            present,
        )];
        assert_eq!(fact_seed(&rule, &negated), None);
    }
}
