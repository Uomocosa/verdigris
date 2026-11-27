use std::collections::BTreeMap;
use std::collections::BTreeSet;

use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;

use crate::Rel;
use crate::Term;
use crate::Value;

#[derive(Debug, Clone, Default, PartialEq, Eq, Deref)]
pub struct Facts {
    pub(crate) tuples: BTreeMap<Rel, BTreeSet<Vec<Value>>>,
}

#[atomic_delegates]
impl Facts {
    #[must_use]
    pub fn add(&mut self, relation: Rel, tuple: Vec<Value>) -> bool {}
    #[must_use]
    pub fn matching(&self, relation: &Rel, pattern: &[Term]) -> Vec<&Vec<Value>> {}
    #[must_use]
    pub fn iter(&self) -> impl Iterator<Item = (Rel, &[Value])> + '_ {}
}

#[rustfmt::skip]
impl Facts {
    #[must_use]
    pub fn contains(&self, relation: &Rel, tuple: &[Value]) -> bool { self.tuples.get(relation).is_some_and(|known| known.contains(tuple)) }
}

#[cfg(test)]
mod tests {
    use crate::Rel;
    use crate::Value;

    use super::Facts;

    #[test]
    fn test_usage() {
        let mut facts = Facts::default();
        assert!(facts.add(Rel("edge"), vec![Value::Int(1), Value::Int(2)]));
        assert!(facts.contains(&Rel("edge"), &[Value::Int(1), Value::Int(2)]));
        assert!(!facts.contains(&Rel("edge"), &[Value::Int(2), Value::Int(1)]));
    }
}
