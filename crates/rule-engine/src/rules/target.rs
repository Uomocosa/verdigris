use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Target {
    Event(rules::EventId),
    Restriction(rules::RestrictionId),
    Definition(rules::DefinitionId),
    Value(rules::RuleId),
}

impl Target {
    #[must_use]
    pub const fn event<E: rules::traits::event::Event>() -> Self {
        Self::Event(E::ID)
    }

    #[must_use]
    pub const fn value<V: rules::traits::value_rule::ValueRule>() -> Self {
        Self::Value(V::ID)
    }
}

#[cfg(test)]
mod tests {
    use crate::events;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::value_rule::ValueRule;

    #[test]
    fn test_usage() {
        let event = rules::Target::event::<events::Draw>();
        assert!(matches!(event, rules::Target::Event(id) if id == events::Draw::ID));
        let value = rules::Target::value::<rules::MaximumHandSize>();
        assert!(matches!(value, rules::Target::Value(id) if id == rules::MaximumHandSize::ID));
    }
}
