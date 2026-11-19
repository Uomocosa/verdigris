use bevy::prelude::Entity;
use derive_more::Deref;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deref)]
pub struct TestCard(#[deref] pub(crate) Entity);

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
    }
}
