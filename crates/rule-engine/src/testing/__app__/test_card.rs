use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::Entity;
use derive_more::Deref;

use crate::cards;
use crate::testing;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deref)]
pub struct TestCard(#[deref] pub(crate) Entity);

#[atomic_delegates]
impl TestCard {
    #[must_use]
    pub fn mana_ability(self, produces: cards::Color) -> testing::TestAbility {}
}

#[rustfmt::skip]
impl TestCard {
    #[must_use]
    pub fn ability(self) -> testing::TestAbility { testing::TestAbility::Only { source: *self } }
}

#[rustfmt::skip]
impl From<TestCard> for Entity {
    fn from(card: TestCard) -> Self { *card }
}

#[rustfmt::skip]
impl PartialEq<TestCard> for Entity {
    fn eq(&self, other: &TestCard) -> bool { *self == **other }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{Entity, World};

    use crate::testing;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let entity = world.spawn_empty().id();
        let card = testing::TestCard(entity);

        assert_eq!(Entity::from(card), entity);
        assert_eq!(*card, entity);
        assert_eq!(
            card.ability(),
            testing::TestAbility::Only { source: entity }
        );
    }
}
