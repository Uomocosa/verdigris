use std::collections::BTreeMap;
use std::collections::BTreeSet;

use datafrog::Relation;

use crate::Atom;
use crate::Literal;
use crate::Program;
use crate::Rel;
use crate::Rule;
use crate::Term;
use crate::Value;

pub fn count_rows(rule: &Rule, completed: &BTreeMap<Rel, Relation<Vec<Value>>>) -> Vec<Vec<Value>> {
    let Some(Literal::Count { body, .. }) =
        rule.body.iter().find(|literal| matches!(literal, Literal::Count { .. }))
    else {
        return Vec::new();
    };
    let groups = Program::count_group_variables(rule);
    let mut counted = BTreeSet::new();
    for atom in body {
        for term in &atom.terms {
            if let Term::Var(name) = term {
                counted.insert(*name);
            }
        }
    }
    let counted: Vec<&str> = counted.into_iter().collect();
    let bind_atom = |atom: &Atom,
                     tuple: &[Value],
                     prior: &BTreeMap<&'static str, Value>|
     -> Option<BTreeMap<&'static str, Value>> {
        if tuple.len() != atom.terms.len() {
            return None;
        }
        let mut bindings = prior.clone();
        for (position, term) in atom.terms.iter().enumerate() {
            match term {
                Term::Val(expected) => {
                    if tuple.get(position) != Some(expected) {
                        return None;
                    }
                }
                Term::Var(name) => {
                    let value = *tuple.get(position)?;
                    if let Some(bound) = bindings.get(name)
                        && *bound != value
                    {
                        return None;
                    }
                    bindings.insert(*name, value);
                }
            }
        }
        Some(bindings)
    };
    let mut rows: Vec<BTreeMap<&'static str, Value>> = vec![BTreeMap::new()];
    for atom in body {
        let Some(relation) = completed.get(&atom.rel) else {
            return Vec::new();
        };
        let mut next = Vec::new();
        for bindings in &rows {
            for tuple in relation.iter() {
                if let Some(bound) = bind_atom(atom, tuple, bindings) {
                    next.push(bound);
                }
            }
        }
        rows = next;
    }
    let mut by_group: BTreeMap<Vec<Value>, BTreeSet<Vec<Value>>> = BTreeMap::new();
    for bindings in &rows {
        let Some(key) =
            groups.iter().map(|name| bindings.get(name).copied()).collect::<Option<Vec<Value>>>()
        else {
            continue;
        };
        let projection: Vec<Value> =
            counted.iter().filter_map(|name| bindings.get(name).copied()).collect();
        by_group.entry(key).or_default().insert(projection);
    }
    by_group
        .into_iter()
        .map(|(key, projections)| {
            let count = i64::try_from(projections.len()).unwrap_or(i64::MAX);
            let mut row = key;
            row.push(Value::Int(count));
            row
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use datafrog::Relation;

    use crate::Atom;
    use crate::Literal;
    use crate::Rel;
    use crate::Rule;
    use crate::Term;
    use crate::Value;

    use super::count_rows;

    #[test]
    fn test_usage() {
        let rule = Rule {
            head: Atom {
                rel: Rel("counted"),
                terms: vec![Term::Var("P"), Term::Var("N")],
            },
            body: vec![Literal::Count {
                output: "N",
                body: vec![Atom {
                    rel: Rel("member"),
                    terms: vec![Term::Var("C"), Term::Var("P")],
                }],
            }],
        };
        let completed = BTreeMap::from([(
            Rel("member"),
            Relation::from_vec(vec![
                vec![Value::Int(1), Value::Int(10)],
                vec![Value::Int(2), Value::Int(10)],
                vec![Value::Int(3), Value::Int(20)],
            ]),
        )]);
        assert_eq!(
            count_rows(&rule, &completed),
            vec![
                vec![Value::Int(10), Value::Int(2)],
                vec![Value::Int(20), Value::Int(1)]
            ]
        );
    }
}
