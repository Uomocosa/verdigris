use crate::cards;
use crate::rules;
use crate::testing;

pub fn event_restrictions() -> Vec<rules::Restriction> {
    vec![
        testing::UNTAPPED
            .restriction(
                "must be untapped for any reason",
                (!rules::has::<cards::Tapped>()).of(testing::AbsoluteRule::ATTACKER),
            )
            .unwaivable(),
    ]
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::rules;
    use crate::testing;

    use super::event_restrictions;

    #[test]
    fn test_usage() {
        let restrictions = event_restrictions();
        assert_eq!(restrictions.len(), 1);
        assert_eq!(restrictions[0].id, rules::RestrictionId("test/untapped"));
        assert_eq!(
            restrictions[0].condition,
            (!rules::has::<cards::Tapped>()).of(testing::AbsoluteRule::ATTACKER)
        );
        assert!(restrictions[0].unwaivable);
    }
}
