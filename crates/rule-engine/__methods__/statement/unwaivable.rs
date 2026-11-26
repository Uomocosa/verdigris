use crate::rules;

pub fn unwaivable<Id>(statement: rules::Statement<Id>) -> rules::Statement<Id> {
    rules::Statement {
        unwaivable: true,
        ..statement
    }
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::rules;

    use super::unwaivable;

    #[test]
    fn test_usage() {
        let restriction = rules::RestrictionId("509.1a/untapped").restriction(
            "The chosen creatures must be untapped",
            rules::Role::Subject,
            !rules::has::<cards::Tapped>(),
        );
        assert!(!restriction.unwaivable);
        let made_unwaivable = unwaivable(restriction);
        assert!(made_unwaivable.unwaivable);
        assert_eq!(made_unwaivable.id, rules::RestrictionId("509.1a/untapped"));
    }
}
