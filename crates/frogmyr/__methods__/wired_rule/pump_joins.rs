use crate::Value;
use datafrog::PrefixFilter;

use crate::Payload;
use crate::WiredRule;

pub fn pump_joins(rule: &WiredRule) {
    if rule.plan.positives.is_empty() {
        return;
    }
    for (index, step) in rule.steps.iter().enumerate() {
        let Some(keyed) = rule.keyed.get(index) else {
            return;
        };
        let Some(source) = rule.sources.get(index) else {
            return;
        };
        let Some(columns) = rule.key_columns.get(index) else {
            return;
        };
        let previous = if index == 0 {
            &rule.unit
        } else {
            let Some(previous) = rule.steps.get(index.saturating_sub(1)) else {
                return;
            };
            previous
        };
        keyed.from_map(source, |row| {
            (
                columns
                    .iter()
                    .map(|position| row.get(*position).copied().unwrap_or(Value::Int(0)))
                    .collect(),
                (*row).clone(),
            )
        });
        step.from_join(previous, keyed, |_key, prior, atom_row| {
            rule.join_step(index, prior, atom_row).unwrap_or_else(|| {
                (
                    Vec::new(),
                    Payload {
                        alive: false,
                        bindings: vec![None; rule.plan.variables.len()],
                    },
                )
            })
        });
    }
    let Some(last) = rule.steps.last() else {
        return;
    };
    rule.candidates.from_map(last, |(_, payload)| (*payload).clone());
    rule.filtered.from_leapjoin(
        &rule.candidates,
        PrefixFilter::from(|payload: &Payload| {
            payload.alive
                && payload.bindings.iter().all(Option::is_some)
                && rule
                    .plan
                    .comparisons
                    .iter()
                    .all(|comparison| rule.plan.comparison_holds(comparison, &payload.bindings))
        }),
        |payload, ()| (*payload).clone(),
    );
}

#[cfg(test)]
mod tests {
    use crate::Atom;
    use crate::Literal;
    use crate::Rel;
    use crate::Rule;
    use crate::Term;
    use crate::Value;
    use datafrog::Iteration;

    use super::pump_joins;
    use crate::CompiledRule;
    use crate::WiredRule;

    #[test]
    fn test_usage() {
        let plan = CompiledRule::from(&Rule {
            head: Atom {
                rel: Rel("one"),
                terms: vec![Term::Var("X")],
            },
            body: vec![Literal::Pos(Atom {
                rel: Rel("zero"),
                terms: vec![Term::Var("X")],
            })],
        });
        let mut iteration = Iteration::new();
        let zero_rows = iteration.variable::<Vec<Value>>("zero");
        zero_rows.extend([vec![Value::Int(5)]]);
        let head_rows = iteration.variable::<Vec<Value>>("head");
        let wired = WiredRule::from_plan(
            plan,
            &mut iteration,
            &head_rows,
            &[zero_rows],
            &std::collections::BTreeMap::default(),
        );
        while iteration.changed() {
            pump_joins(&wired);
            wired.pump_heads();
        }
        let candidate_count: usize =
            wired.candidates.stable.borrow().iter().map(|batch| batch.len()).sum();
        let filtered_count: usize =
            wired.filtered.stable.borrow().iter().map(|batch| batch.len()).sum();
        assert_eq!(candidate_count, 1);
        assert_eq!(filtered_count, 1);
    }
}
