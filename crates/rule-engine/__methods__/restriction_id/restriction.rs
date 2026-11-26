use crate::rules;

pub const fn restriction(
    restriction_id: rules::RestrictionId,
    condition: rules::Condition,
) -> rules::Restriction {
    rules::Restriction {
        id: restriction_id,
        condition,
        unwaivable: false,
    }
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::rules;

    use super::restriction;

    #[test]
    fn test_usage() {
        let id = rules::RestrictionId("509.1a/untapped");
        let built = restriction(id, !rules::has::<cards::Tapped>());
        assert_eq!(built.id, rules::RestrictionId("509.1a/untapped"));
        assert_eq!(built.condition, !rules::has::<cards::Tapped>());
    }
}
