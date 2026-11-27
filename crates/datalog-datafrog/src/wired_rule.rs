use std::collections::BTreeMap;

use atomic_delegate_macros::atomic_delegates;
use datafrog::Iteration;
use datafrog::Relation;
use datafrog::Variable;
use verdigris_datalog_ir::Atom;
use verdigris_datalog_ir::Rel;
use verdigris_datalog_ir::Value;

use crate::CompiledRule;
use crate::Payload;

pub struct WiredRule {
    pub(crate) plan: CompiledRule,
    pub(crate) unit: Variable<(Vec<Value>, Payload)>,
    pub(crate) steps: Vec<Variable<(Vec<Value>, Payload)>>,
    pub(crate) sources: Vec<Variable<Vec<Value>>>,
    pub(crate) keyed: Vec<Variable<(Vec<Value>, Vec<Value>)>>,
    pub(crate) key_columns: Vec<Vec<usize>>,
    pub(crate) next_columns: Vec<Vec<usize>>,
    pub(crate) constant_terms: Vec<Vec<(usize, Value)>>,
    pub(crate) duplicate_terms: Vec<Vec<(usize, usize)>>,
    pub(crate) candidates: Variable<Payload>,
    pub(crate) filtered: Variable<Payload>,
    pub(crate) negated: Vec<(Atom, Relation<Vec<Value>>)>,
    pub(crate) neg_staged: Vec<Variable<(Vec<Value>, Payload)>>,
    pub(crate) neg_outputs: Vec<Variable<Payload>>,
    pub(crate) head_rows: Variable<Vec<Value>>,
}

#[atomic_delegates]
impl WiredRule {
    pub(crate) fn from_plan(
        plan: CompiledRule,
        iteration: &mut Iteration,
        head_rows: &Variable<Vec<Value>>,
        sources: &[Variable<Vec<Value>>],
        completed: &BTreeMap<Rel, Relation<Vec<Value>>>,
    ) -> Self {
    }
}

#[atomic_delegates]
impl WiredRule {
    pub(crate) fn pump(&self) {}
    pub(crate) fn pump_joins(&self) {}
    pub(crate) fn pump_heads(&self) {}
    pub(crate) fn join_step(
        &self,
        index: usize,
        prior: &Payload,
        atom_row: &[Value],
    ) -> Option<(Vec<Value>, Payload)> {
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

    use super::WiredRule;
    use crate::CompiledRule;

    #[test]
    fn test_usage() {
        let plan = CompiledRule::from(&Rule {
            head: Atom {
                rel: Rel("one"),
                terms: vec![Term::Val(Value::Int(1))],
            },
            body: vec![Literal::Pos(Atom {
                rel: Rel("zero"),
                terms: vec![Term::Val(Value::Int(0))],
            })],
        });
        let mut iteration = datafrog::Iteration::new();
        let head_rows = iteration.variable::<Vec<Value>>("head");
        let wired = WiredRule::from_plan(
            plan,
            &mut iteration,
            &head_rows,
            &[],
            &std::collections::BTreeMap::default(),
        );
        wired.pump();
        assert_eq!(wired.steps.len(), 1);
    }
}
