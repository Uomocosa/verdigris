use crate::WiredRule;

pub fn pump(rule: &WiredRule) {
    rule.pump_joins();
    rule.pump_heads();
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

    use super::pump;
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
            pump(&wired);
        }
        let headed: usize = wired.head_rows.stable.borrow().iter().map(|batch| batch.len()).sum();
        assert_eq!(headed, 1);
    }
}
