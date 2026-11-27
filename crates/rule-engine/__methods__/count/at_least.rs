use crate::rules;

pub fn at_least(count: rules::Count, threshold: u32) -> rules::Condition {
    rules::Condition::Count(
        *count,
        rules::Comparison::OrMore(i32::try_from(threshold).unwrap_or(i32::MAX)),
    )
}

#[cfg(test)]
mod tests {
    use crate::events;
    use crate::rules;

    use super::at_least;

    #[test]
    fn test_usage() {
        assert_eq!(
            at_least(rules::count(events::DeclareAttackers::ATTACKERS), 2),
            rules::Condition::Count(
                events::DeclareAttackers::ATTACKERS,
                rules::Comparison::OrMore(2),
            )
        );
    }
}
