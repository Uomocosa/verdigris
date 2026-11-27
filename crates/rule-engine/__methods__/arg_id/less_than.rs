use crate::rules;

pub fn less_than(arg_id: rules::ArgId, threshold: u32) -> rules::Condition {
    rules::Condition::ArgAmount(
        arg_id,
        rules::Comparison::LessThan(i32::try_from(threshold).unwrap_or(i32::MAX)),
    )
}

#[cfg(test)]
mod tests {
    use crate::rules;

    use super::less_than;

    #[test]
    fn test_usage() {
        let id = rules::ArgId("source");
        assert_eq!(
            less_than(id, 2),
            rules::Condition::ArgAmount(id, rules::Comparison::LessThan(2))
        );
    }
}
