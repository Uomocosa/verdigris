use crate::events;
use crate::rules;

pub const fn from_this(_: rules::This) -> rules::Subject {
    rules::Subject::One(rules::ArgRef::SourceOf(
        events::AbilityContext::SPELL_OR_ABILITY,
    ))
}

#[cfg(test)]
mod tests {
    use crate::events;
    use crate::rules;

    use super::from_this;

    #[test]
    fn test_usage() {
        assert_eq!(
            from_this(rules::This),
            rules::Subject::One(rules::ArgRef::SourceOf(
                events::AbilityContext::SPELL_OR_ABILITY,
            ))
        );
    }
}
