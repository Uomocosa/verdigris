use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;

use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct RestrictionId(pub &'static str);

#[atomic_delegates]
impl RestrictionId {
    #[must_use]
    pub fn restriction(
        self,
        text: &'static str,
        condition: rules::Condition,
    ) -> rules::Restriction {
    }
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::rules;

    use super::RestrictionId;

    #[test]
    fn test_usage() {
        let id = RestrictionId("509.1a/untapped");
        let built = id.restriction(
            "The chosen creatures must be untapped",
            !rules::has::<cards::Tapped>(),
        );
        assert_eq!(built.id, RestrictionId("509.1a/untapped"));
        assert_eq!(built.text, "The chosen creatures must be untapped");
        assert_eq!(built.condition, !rules::has::<cards::Tapped>());
    }
}
