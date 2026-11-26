use atomic_delegate_macros::atomic_delegates;

use crate::rules;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Restriction {
    pub id: rules::RestrictionId,
    pub text: &'static str,
    pub role: rules::Role,
    pub condition: rules::Condition,
    pub unwaivable: bool,
}

#[atomic_delegates]
impl Restriction {
    #[must_use]
    pub fn unwaivable(self) -> Self {}
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::rules;

    use super::Restriction;

    #[test]
    fn test_usage() {
        let restriction = rules::RestrictionId("509.1a/untapped").restriction(
            "The chosen creatures must be untapped",
            rules::Role::Subject,
            !rules::has::<cards::Tapped>(),
        );
        assert_eq!(restriction.id, rules::RestrictionId("509.1a/untapped"));
        assert!(!restriction.unwaivable);
        let made_unwaivable = Restriction {
            unwaivable: true,
            ..restriction
        };
        assert!(made_unwaivable.unwaivable);
    }
}
