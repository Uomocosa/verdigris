use atomic_delegate_macros::atomic_delegates;

use crate::cards;
use crate::keywords;
use crate::rules;
use crate::turns;
use crate::zones;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Condition {
    Has(rules::ComponentRef),
    HasKeyword(keywords::KeywordId),
    PointsTo(rules::ComponentRef, rules::ArgRef),
    Is(rules::ArgRef),
    CardType(cards::CardType),
    Characteristic(rules::Characteristic, rules::Comparison, i32),
    Of(rules::ArgRef, Box<Self>),
    All(Vec<Self>),
    Any(Vec<Self>),
    Not(Box<Self>),
    True,
    InZone(zones::Zone, rules::ArgRef),
    StepIs(turns::Step),
    StackEmpty,
    ActivePlayer(rules::ArgRef),
    HasPriority(rules::ArgRef),
    ArgAmount(rules::ArgId, rules::Comparison, u32),
    ArgAtLeastRemainingToughness(rules::ArgId),
    Count(rules::ArgId, rules::Comparison, u32),
}

#[rustfmt::skip]
impl std::ops::Not for Condition {
    type Output = Self;
    fn not(self) -> Self { Self::Not(Box::new(self)) }
}

impl std::ops::BitAnd for Condition {
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

impl std::ops::BitOr for Condition {
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
impl Condition {
    #[must_use]
    pub fn bind<E: rules::traits::event_args::EventArgs>(&self, event: &E) -> Self {}
}

#[rustfmt::skip]
impl Condition {
    #[must_use]
    pub fn of(self, who: impl Into<rules::ArgRef>) -> Self { Self::Of(who.into(), Box::new(self)) }
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::rules;

    use super::Condition;

    #[test]
    fn test_usage() {
        let tapped = rules::has::<cards::Tapped>();
        assert_eq!(
            tapped,
            rules::Condition::Has(rules::ComponentRef::of::<cards::Tapped>())
        );
        let combined = tapped.clone() & Condition::True;
        assert_eq!(combined, Condition::All(vec![tapped, Condition::True]));
    }
}
