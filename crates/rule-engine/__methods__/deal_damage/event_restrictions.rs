use crate::events;
use crate::rules;

pub fn event_restrictions() -> Vec<rules::Restriction> {
    vec![
        events::deal_damage::NONZERO_DAMAGE
            .restriction(events::DealDamage::AMOUNT.is(rules::Value(1).or_more())),
    ]
}

#[cfg(test)]
mod tests {
    use crate::events;
    use crate::rules;

    use super::event_restrictions;

    #[test]
    fn test_usage() {
        let restrictions = event_restrictions();
        assert_eq!(restrictions.len(), 1);
        assert_eq!(restrictions[0].id, rules::RestrictionId("120.8/nonzero"));
        assert_eq!(
            restrictions[0].condition,
            events::DealDamage::AMOUNT.is(rules::Value(1).or_more())
        );
    }
}
