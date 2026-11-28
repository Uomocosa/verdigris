use std::fmt::Display;

use atomic_delegate_macros::atomic_delegates;

use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RestrictionId {
    pub card: &'static str,
    pub kind: &'static str,
    pub event: rules::EventId,
}

impl RestrictionId {
    pub const UNSET: &'static str = "";
}

#[atomic_delegates]
impl RestrictionId {
    #[must_use]
    pub const fn rule(id: &'static str) -> Self {}
}

impl Display for RestrictionId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.kind.is_empty() {
            write!(formatter, "{}", self.card)
        } else {
            write!(formatter, "{}/{}/{}", self.card, self.kind, *self.event)
        }
    }
}

#[atomic_delegates]
impl RestrictionId {
    #[must_use]
    pub fn restriction(self, condition: rules::Condition) -> rules::Restriction {}
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::events;
    use crate::rules;
    use crate::rules::traits::event::Event;

    use super::RestrictionId;

    #[test]
    fn test_usage() {
        let id = RestrictionId::rule("509.1a/untapped");
        let built = id.restriction(!rules::has::<cards::Tapped>());
        assert_eq!(built.id, RestrictionId::rule("509.1a/untapped"));
        assert_eq!(built.condition, !rules::has::<cards::Tapped>());
        assert_eq!(id.to_string(), "509.1a/untapped");
    }

    #[test]
    fn card_restriction_displays_card_kind_and_event() {
        let id = RestrictionId {
            card: "test_safe_haven",
            kind: "cant",
            event: events::Destroy::ID,
        };
        assert_eq!(id.to_string(), "test_safe_haven/cant/701.8a/destroy");
    }
}
