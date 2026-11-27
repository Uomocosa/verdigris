use crate::rules;

pub fn greater_than(arg_id: rules::ArgId, threshold: u32) -> rules::Condition {
    rules::Condition::ArgAmount(
        arg_id,
        rules::Comparison::GreaterThan(i32::try_from(threshold).unwrap_or(i32::MAX)),
    )
}

#[cfg(test)]
mod tests {
    use crate::rules;

    use super::greater_than;

    #[test]
    fn test_usage() {
        let id = rules::ArgId("source");
        assert_eq!(
            greater_than(id, 0),
            rules::Condition::ArgAmount(id, rules::Comparison::GreaterThan(0))
        );
    }
}
