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

    #[test]
    fn test_examples_facts() {
        use crate::Term;
        let mut facts = Facts::default();
        assert!(facts.add(Rel("creature"), vec![Value::Sym("bears")]));
        assert!(facts.add(Rel("creature"), vec![Value::Sym("goblin")]));
        assert!(facts.add(Rel("tapped"), vec![Value::Sym("bears")]));
        println!("1. facts in:");
        for (relation, tuple) in facts.iter() {
            println!("   {}({tuple:?})", *relation);
        }
        let matched = facts.matching(&Rel("tapped"), &[Term::Var("C")]);
        println!("2. matching tapped(C) finds {} row(s):", matched.len());
        for row in &matched {
            println!("   tapped({row:?})");
        }
        assert_eq!(matched, vec![&vec![Value::Sym("bears")]]);
        let none = facts.matching(&Rel("tapped"), &[Term::Val(Value::Sym("goblin"))]);
        println!("3. matching tapped(goblin) finds {} row(s)", none.len());
        assert_eq!(none, Vec::<&Vec<Value>>::new());
    }
}
