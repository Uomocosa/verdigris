use atomic_delegate_macros::atomic_delegates;

use crate::rules;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Statement<Id> {
    pub id: Id,
    pub condition: rules::Condition,
    pub unwaivable: bool,
}

#[atomic_delegates]
impl Statement<rules::DefinitionId> {
    #[must_use]
    pub fn lethal_damage() -> Self {}
}

#[atomic_delegates]
impl<Id> Statement<Id> {
    #[must_use]
    pub fn unwaivable(self) -> Self {}
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::rules;

    use super::Statement;

    #[test]
    fn test_usage() {
        let restriction = rules::RestrictionId::rule("509.1a/untapped")
            .restriction(!rules::has::<cards::Tapped>());
        assert_eq!(
            restriction.id,
            rules::RestrictionId::rule("509.1a/untapped")
        );
        assert!(!restriction.unwaivable);
        let made_unwaivable: Statement<rules::RestrictionId> = restriction.unwaivable();
        assert!(made_unwaivable.unwaivable);
        assert_eq!(
            made_unwaivable.id,
            rules::RestrictionId::rule("509.1a/untapped")
        );
    }
}
