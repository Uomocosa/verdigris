use atomic_delegate_macros::atomic_delegates;

use crate::cards;
use crate::rules;
use crate::testing;

#[derive(Clone, Copy, Debug)]
pub struct Card {
    pub name: testing::TestCardName,
    pub definition: fn() -> cards::CardDefinition,
}

#[atomic_delegates]
impl Card {
    #[must_use]
    pub fn restriction<E: rules::traits::event::Event>(&self) -> Option<rules::RestrictionId> {}
}

#[rustfmt::skip]
impl Card {
    #[must_use]
    pub fn trigger(self) -> testing::TriggerQuery { self.into() }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;

    #[test]
    fn test_usage() {
        let query = SOUL_WARDEN.trigger();
        assert_eq!(query.label(), "Soul Warden");
        assert_eq!(CardQuery::from(SOUL_WARDEN).label(), "Soul Warden");
    }
}
