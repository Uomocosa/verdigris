use crate::rules;

pub fn at_least(arg_id: rules::ArgId, threshold: u32) -> rules::Condition {
    rules::Condition::ArgAmount(
        arg_id,
        rules::Comparison::OrMore(i32::try_from(threshold).unwrap_or(i32::MAX)),
    )
}

#[cfg(test)]
mod tests {
    use crate::rules;

    use super::at_least;

    #[test]
    fn test_usage() {
        let id = rules::ArgId("source");
        assert_eq!(
            at_least(id, 2),
            rules::Condition::ArgAmount(id, rules::Comparison::OrMore(2))
        );
    }
}
