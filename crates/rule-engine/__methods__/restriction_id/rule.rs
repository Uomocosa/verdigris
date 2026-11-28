use crate::rules;

pub const fn rule(id: &'static str) -> rules::RestrictionId {
    rules::RestrictionId {
        card: id,
        kind: "",
        event: rules::EventId(""),
    }
}

#[cfg(test)]
mod tests {
    use crate::rules;

    use super::rule;

    #[test]
    fn test_usage() {
        let id = rule("509.1a/untapped");
        assert_eq!(id.to_string(), "509.1a/untapped");
        assert_eq!(id, rules::RestrictionId::rule("509.1a/untapped"));
    }
}
