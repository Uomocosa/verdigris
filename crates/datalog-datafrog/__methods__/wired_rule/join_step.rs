use verdigris_datalog_ir::Term;
use verdigris_datalog_ir::Value;

use crate::Payload;
use crate::WiredRule;

pub fn join_step(
    rule: &WiredRule,
    index: usize,
    prior: &Payload,
    atom_row: &[Value],
) -> Option<(Vec<Value>, Payload)> {
    if !prior.alive {
        return None;
    }
    let constants = rule.constant_terms.get(index)?;
    let duplicates = rule.duplicate_terms.get(index)?;
    let next = rule.next_columns.get(index)?;
    let atom = rule.plan.positives.get(index)?;
    for (position, expected) in constants {
        let actual = atom_row.get(*position)?;
        if actual != expected {
            return None;
        }
    }
    for (first, second) in duplicates {
        let (Some(one), Some(other)) = (atom_row.get(*first), atom_row.get(*second)) else {
            return None;
        };
        if one != other {
            return None;
        }
    }
    let mut merged = prior.bindings.clone();
    for (position, term) in atom.terms.iter().enumerate() {
        if let Term::Var(name) = term {
            let found = rule.plan.positions.get(name)?;
            let value = atom_row.get(position)?;
            let slot = merged.get_mut(*found)?;
            *slot = Some(*value);
        }
    }
    let mut key = Vec::with_capacity(next.len());
    for position in next {
        let bound = merged.get(*position).and_then(|slot| *slot)?;
        key.push(bound);
    }
    Some((
        key,
        Payload {
            alive: true,
            bindings: merged,
        },
    ))
}

#[cfg(test)]
mod tests {
    use datafrog::Iteration;
    use verdigris_datalog_ir::Atom;
    use verdigris_datalog_ir::Literal;
    use verdigris_datalog_ir::Rel;
    use verdigris_datalog_ir::Rule;
    use verdigris_datalog_ir::Term;
    use verdigris_datalog_ir::Value;

    use super::join_step;
    use crate::CompiledRule;
    use crate::Payload;
    use crate::WiredRule;

    #[test]
    fn test_usage() {
        let plan = CompiledRule::from(&Rule {
            head: Atom {
                rel: Rel("from_one"),
                terms: vec![Term::Var("X")],
            },
            body: vec![Literal::Pos(Atom {
                rel: Rel("edge"),
                terms: vec![Term::Val(Value::Int(1)), Term::Var("X")],
            })],
        });
        let mut iteration = Iteration::new();
        let head_rows = iteration.variable::<Vec<Value>>("head");
        let wired = WiredRule::from_plan(
            plan,
            &mut iteration,
            &head_rows,
            &[],
            &std::collections::BTreeMap::default(),
        );
        let prior = Payload {
            alive: true,
            bindings: vec![None],
        };
        let (key, payload) = join_step(&wired, 0, &prior, &[Value::Int(1), Value::Int(2)]).unwrap();
        assert_eq!(key, Vec::new());
        assert!(payload.alive);
        assert_eq!(payload.bindings, vec![Some(Value::Int(2))]);
        let dead = Payload {
            alive: false,
            bindings: vec![None],
        };
        assert_eq!(
            join_step(&wired, 0, &dead, &[Value::Int(1), Value::Int(2)]),
            None
        );
        assert_eq!(
            join_step(&wired, 0, &prior, &[Value::Int(9), Value::Int(2)]),
            None
        );
    }
}
