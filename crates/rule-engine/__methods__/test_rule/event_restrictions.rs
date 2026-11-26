use crate::cards;
use crate::rules;
use crate::testing;

pub fn event_restrictions() -> Vec<rules::Restriction> {
    vec![
        testing::UNTAPPED
            .restriction((!rules::has::<cards::Tapped>()).of(testing::TestRule::ATTACKER)),
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
        assert_eq!(restrictions[0].id, testing::UNTAPPED);
        assert_eq!(
            restrictions[0].condition,
            (!rules::has::<cards::Tapped>()).of(testing::TestRule::ATTACKER)
        );
    }
}
