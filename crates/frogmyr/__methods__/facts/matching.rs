use crate::Facts;
use crate::Rel;
use crate::Term;
use crate::Value;

pub fn matching<'facts>(
    facts: &'facts Facts,
    relation: &Rel,
    pattern: &[Term],
) -> Vec<&'facts Vec<Value>> {
    let Facts { tuples } = facts;
    let Some(known) = tuples.get(relation) else {
        return Vec::new();
    };
    known
        .iter()
        .filter(|tuple| {
            tuple.len() == pattern.len()
                && tuple.iter().zip(pattern.iter()).all(|(value, term)| match term {
                    Term::Var(_) => true,
                    Term::Val(expected) => value == expected,
                })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::Facts;
    use crate::Rel;
    use crate::Term;
    use crate::Value;

    use super::matching;

    #[test]
    fn test_usage() {
        let mut facts = Facts::default();
        assert!(facts.add(Rel("edge"), vec![Value::Int(1), Value::Int(2)]));
        assert!(facts.add(Rel("edge"), vec![Value::Int(1), Value::Int(3)]));
        assert!(facts.add(Rel("edge"), vec![Value::Int(2), Value::Int(3)]));
        let found = matching(
            &facts,
            &Rel("edge"),
            &[Term::Val(Value::Int(1)), Term::Var("To")],
        );
        assert_eq!(
            found,
            vec![
                &vec![Value::Int(1), Value::Int(2)],
                &vec![Value::Int(1), Value::Int(3)],
            ]
        );
        assert_eq!(
            matching(&facts, &Rel("missing"), &[Term::Var("X")]),
            Vec::<&Vec<Value>>::new()
        );
    }
}
