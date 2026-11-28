use crate::Facts;
use crate::Rel;
use crate::Value;

pub fn iter(facts: &Facts) -> impl Iterator<Item = (Rel, &[Value])> + '_ {
    let Facts { tuples } = facts;
    tuples
        .iter()
        .flat_map(|(relation, known)| known.iter().map(|tuple| (*relation, tuple.as_slice())))
}

#[cfg(test)]
mod tests {
    use crate::Facts;
    use crate::Rel;
    use crate::Value;

    use super::iter;

    #[test]
    fn test_usage() {
        let mut facts = Facts::default();
        assert!(facts.add(Rel("b"), vec![Value::Int(2)]));
        assert!(facts.add(Rel("a"), vec![Value::Int(1)]));
        assert_eq!(
            iter(&facts).collect::<Vec<_>>(),
            vec![
                (Rel("a"), [Value::Int(1)].as_slice()),
                (Rel("b"), [Value::Int(2)].as_slice()),
            ]
        );
    }
}
