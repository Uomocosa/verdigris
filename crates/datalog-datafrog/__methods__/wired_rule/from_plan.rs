use std::collections::BTreeMap;
use std::collections::BTreeSet;

use datafrog::Iteration;
use datafrog::Relation;
use datafrog::Variable;
use verdigris_datalog_ir::Rel;
use verdigris_datalog_ir::Term;
use verdigris_datalog_ir::Value;

use crate::Payload;

use crate::CompiledRule;
use crate::WiredRule;

pub fn from_plan(
    plan: CompiledRule,
    iteration: &mut Iteration,
    head_rows: &Variable<Vec<Value>>,
    sources: &[Variable<Vec<Value>>],
    completed: &BTreeMap<Rel, Relation<Vec<Value>>>,
) -> WiredRule {
    let mut key_columns: Vec<Vec<usize>> = Vec::new();
    let mut next_columns: Vec<Vec<usize>> = Vec::new();
    let mut constant_terms: Vec<Vec<(usize, Value)>> = Vec::new();
    let mut duplicate_terms: Vec<Vec<(usize, usize)>> = Vec::new();
    let mut bound: BTreeSet<&'static str> = BTreeSet::new();
    for (index, atom) in plan.positives.iter().enumerate() {
        let mut seen: BTreeMap<&'static str, Vec<usize>> = BTreeMap::new();
        let mut constants: Vec<(usize, Value)> = Vec::new();
        for (position, term) in atom.terms.iter().enumerate() {
            match term {
                Term::Val(value) => constants.push((position, *value)),
                Term::Var(name) => seen.entry(*name).or_default().push(position),
            }
        }
        let mut keys: Vec<usize> = Vec::new();
        for name in &plan.variables {
            if bound.contains(name)
                && let Some(positions) = seen.get(name)
                && let Some(first) = positions.first()
            {
                keys.push(*first);
            }
        }
        let mut duplicates: Vec<(usize, usize)> = Vec::new();
        for positions in seen.values() {
            for pair in positions.windows(2) {
                if let [first, second] = pair {
                    duplicates.push((*first, *second));
                }
            }
        }
        for name in seen.keys() {
            bound.insert(*name);
        }
        let mut next: Vec<usize> = Vec::new();
        if let Some(following) = plan.positives.get(index.saturating_add(1)) {
            for name in &plan.variables {
                if bound.contains(name)
                    && following.terms.iter().any(|term| term == &Term::Var(name))
                    && let Some(position) = plan.positions.get(name)
                {
                    next.push(*position);
                }
            }
        }
        key_columns.push(keys);
        next_columns.push(next);
        constant_terms.push(constants);
        duplicate_terms.push(duplicates);
    }
    let arity = plan.variables.len();
    let unit: Variable<(Vec<Value>, Payload)> = iteration.variable("unit");
    let bindings = vec![None; arity];
    unit.extend([(
        Vec::new(),
        Payload {
            alive: true,
            bindings,
        },
    )]);
    let mut steps = Vec::with_capacity(plan.positives.len());
    let mut keyed = Vec::with_capacity(plan.positives.len());
    for _ in &plan.positives {
        steps.push(iteration.variable::<(Vec<Value>, Payload)>("step"));
        keyed.push(iteration.variable("keyed"));
    }
    let filtered: Variable<Payload> = iteration.variable("filtered");
    let candidates: Variable<Payload> = iteration.variable("candidates");
    let negated = plan.ground_negations(completed);
    let mut neg_staged = Vec::new();
    let mut neg_outputs = Vec::new();
    for _ in &plan.negatives {
        neg_staged.push(iteration.variable("neg"));
        neg_outputs.push(iteration.variable("neg"));
    }
    if let Some(seed) = plan.fact_seed(&negated) {
        head_rows.extend([seed]);
    }
    WiredRule {
        plan,
        unit,
        steps,
        sources: sources.to_vec(),
        keyed,
        key_columns,
        next_columns,
        constant_terms,
        duplicate_terms,
        candidates,
        filtered,
        negated,
        neg_staged,
        neg_outputs,
        head_rows: head_rows.clone(),
    }
}

#[cfg(test)]
mod tests {
    use verdigris_datalog_ir::Atom;
    use verdigris_datalog_ir::Literal;
    use verdigris_datalog_ir::Rel;
    use verdigris_datalog_ir::Rule;
    use verdigris_datalog_ir::Term;
    use verdigris_datalog_ir::Value;

    use super::from_plan;
    use crate::CompiledRule;

    #[test]
    fn test_usage() {
        let plan = CompiledRule::from(&Rule {
            head: Atom {
                rel: Rel("path"),
                terms: vec![Term::Var("X"), Term::Var("Z")],
            },
            body: vec![
                Literal::Pos(Atom {
                    rel: Rel("path"),
                    terms: vec![Term::Var("X"), Term::Var("Y")],
                }),
                Literal::Pos(Atom {
                    rel: Rel("edge"),
                    terms: vec![Term::Var("Y"), Term::Var("Z")],
                }),
            ],
        });
        let mut iteration = datafrog::Iteration::new();
        let head_rows = iteration.variable::<Vec<Value>>("head");
        let wired = from_plan(
            plan,
            &mut iteration,
            &head_rows,
            &[],
            &std::collections::BTreeMap::default(),
        );
        assert_eq!(wired.key_columns, vec![vec![], vec![0]]);
        assert_eq!(wired.next_columns, vec![vec![1], vec![]]);
    }
}
