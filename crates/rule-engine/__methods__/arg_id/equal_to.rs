use crate::rules;

pub fn equal_to(arg_id: rules::ArgId, threshold: u32) -> rules::Condition {
    rules::Condition::ArgAmount(
        arg_id,
        rules::Comparison::Exactly(i32::try_from(threshold).unwrap_or(i32::MAX)),
    )
}

#[cfg(test)]
mod tests {
    use crate::rules;

    use super::equal_to;

    #[test]
    fn test_usage() {
        let id = rules::ArgId("source");
        assert_eq!(
            equal_to(id, 2),
            rules::Condition::ArgAmount(id, rules::Comparison::Exactly(2))
        );
    }
}
