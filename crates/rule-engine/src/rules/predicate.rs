use atomic_delegate_macros::atomic_delegates;

use crate::cards;
use crate::rules;
use crate::turns;
use crate::zones;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Predicate {
    Has(rules::ComponentKey),
    PointsTo(rules::ComponentKey, rules::Role),
    CardType(cards::CardType),
    Characteristic(rules::Characteristic, rules::Comparison, i32),
    Of(rules::Role, Box<Self>),
    All(Vec<Self>),
    Any(Vec<Self>),
    Not(Box<Self>),
    True,
    InZone(zones::Zone, rules::Role),
    StepIs(turns::Step),
    StackEmpty,
    ActivePlayer(rules::Role),
    HasPriority(rules::Role),
    EventField(rules::Field, rules::Comparison, u32),
}

#[rustfmt::skip]
impl std::ops::Not for Predicate {
    type Output = Self;
    fn not(self) -> Self { Self::Not(Box::new(self)) }
}

impl std::ops::BitAnd for Predicate {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        let mut conjuncts = match self {
            Self::All(conjuncts) => conjuncts,
            single => vec![single],
        };
        match rhs {
            Self::All(others) => conjuncts.extend(others),
            single => conjuncts.push(single),
        }
        Self::All(conjuncts)
    }
}

impl std::ops::BitOr for Predicate {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        let mut disjuncts = match self {
            Self::Any(disjuncts) => disjuncts,
            single => vec![single],
        };
        match rhs {
            Self::Any(others) => disjuncts.extend(others),
            single => disjuncts.push(single),
        }
        Self::Any(disjuncts)
    }
}

#[atomic_delegates]
impl Predicate {
    #[must_use]
    pub fn bind<E: rules::traits::event::Event>(&self, event: &E) -> Self {}
}

#[cfg(test)]
mod tests {
    use crate::rules;

    use super::Predicate;

    #[test]
    fn test_usage() {
        let tapped = Predicate::Has(rules::ComponentKey("tapped"));
        let combined = tapped.clone() & Predicate::True;
        assert_eq!(combined, Predicate::All(vec![tapped, Predicate::True]));
    }
}
