use crate::Facts;
use crate::Rel;
use crate::Value;

pub fn add(facts: &mut Facts, relation: Rel, tuple: Vec<Value>) -> bool {
    let Facts { tuples } = facts;
    let arity_matches = tuples
        .get(&relation)
        .is_none_or(|known| known.iter().next().is_none_or(|first| first.len() == tuple.len()));
    if !arity_matches {
        return false;
    }
    tuples.entry(relation).or_default().insert(tuple)
}

#[cfg(test)]
mod tests {
    use crate::Facts;
    use crate::Rel;
    use crate::Value;

    use super::add;

    #[test]
    fn test_usage() {
        let mut facts = Facts::default();
        assert!(add(
            &mut facts,
            Rel("edge"),
            vec![Value::Int(1), Value::Int(2)]
        ));
        assert!(!add(&mut facts, Rel("edge"), vec![Value::Int(1)]));
        assert!(facts.contains(&Rel("edge"), &[Value::Int(1), Value::Int(2)]));
    }
}
