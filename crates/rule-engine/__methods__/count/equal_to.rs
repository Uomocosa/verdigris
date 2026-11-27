use crate::rules;

pub fn equal_to(count: rules::Count, threshold: u32) -> rules::Condition {
    rules::Condition::Count(
        *count,
        rules::Comparison::Exactly(i32::try_from(threshold).unwrap_or(i32::MAX)),
    )
}

#[cfg(test)]
mod tests {
    use crate::events;
    use crate::rules;

    use super::equal_to;

    #[test]
    fn test_usage() {
        assert_eq!(
            equal_to(rules::count(events::DeclareAttackers::ATTACKERS), 1),
            rules::Condition::Count(
                events::DeclareAttackers::ATTACKERS,
                rules::Comparison::Exactly(1),
            )
        );
    }
}
