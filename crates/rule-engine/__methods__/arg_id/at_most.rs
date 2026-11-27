use crate::rules;

pub fn at_most(arg_id: rules::ArgId, threshold: u32) -> rules::Condition {
    rules::Condition::ArgAmount(
        arg_id,
        rules::Comparison::OrLess(i32::try_from(threshold).unwrap_or(i32::MAX)),
    )
}

#[cfg(test)]
mod tests {
    use crate::rules;

    use super::at_most;

    #[test]
    fn test_usage() {
        let id = rules::ArgId("source");
        assert_eq!(
            at_most(id, 2),
            rules::Condition::ArgAmount(id, rules::Comparison::OrLess(2))
        );
    }
}
