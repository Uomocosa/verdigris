use crate::WiredRule;

pub fn pump_heads(rule: &WiredRule) {
    if rule.plan.positives.is_empty() {
        return;
    }
    let mut current = &rule.filtered;
    for index in 0..rule.negated.len() {
        let Some((atom, relation)) = rule.negated.get(index) else {
            return;
        };
        let Some(staged) = rule.neg_staged.get(index) else {
            return;
        };
        let Some(output) = rule.neg_outputs.get(index) else {
            return;
        };
        staged.from_map(current, |payload| {
            (
                rule.plan.substitute_all(&atom.terms, &payload.bindings),
                (*payload).clone(),
            )
        });
        output.from_antijoin(staged, relation, |_key, payload| (*payload).clone());
        current = output;
    }
    rule.head_rows.from_map(current, |payload| {
        rule.plan.substitute_all(&rule.plan.head.terms, &payload.bindings)
    });
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

    use super::pump_heads;
    use crate::CompiledRule;
    use crate::WiredRule;

    #[test]
    fn test_usage() {
        let plan = CompiledRule::from(&Rule {
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
        let mut iteration = Iteration::new();
        let query_rows = iteration.variable::<Vec<Value>>("query");
        query_rows.extend([vec![Value::Int(1)], vec![Value::Int(2)]]);
        let head_rows = iteration.variable::<Vec<Value>>("head");
        let wired = WiredRule::from_plan(
            plan,
            &mut iteration,
            &head_rows,
            &[query_rows],
            &std::collections::BTreeMap::default(),
        );
        while iteration.changed() {
            wired.pump_joins();
            pump_heads(&wired);
        }
        let headed: usize = wired.head_rows.stable.borrow().iter().map(|batch| batch.len()).sum();
        assert_eq!(headed, 2);
    }
}
