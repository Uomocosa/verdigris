use crate::rules;

pub fn at_most(count: rules::Count, threshold: u32) -> rules::Condition {
    rules::Condition::Count(
        *count,
        rules::Comparison::OrLess(i32::try_from(threshold).unwrap_or(i32::MAX)),
    )
}

#[cfg(test)]
mod tests {
    use crate::events;
    use crate::rules;

    use super::at_most;

    #[test]
    fn test_usage() {
        assert_eq!(
            at_most(rules::count(events::DeclareAttackers::ATTACKERS), 1),
            rules::Condition::Count(
                events::DeclareAttackers::ATTACKERS,
                rules::Comparison::OrLess(1),
            )
        );
    }
}
