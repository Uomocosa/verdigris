use bevy::prelude::Entity;
use derive_more::Deref;

use crate::testing;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deref)]
pub struct TestPlayer(pub(crate) Entity);

#[rustfmt::skip]
impl From<TestPlayer> for Entity {
    fn from(player: TestPlayer) -> Self { *player }
}

#[rustfmt::skip]
impl PartialEq<TestPlayer> for Entity {
    fn eq(&self, other: &TestPlayer) -> bool { *self == **other }
}

#[rustfmt::skip]
impl TestPlayer {
    #[must_use]
    pub fn cast(self, spell: Entity) -> testing::TestPlayerActions { testing::TestPlayerActions::from_caster_and_spell(*self, spell) }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{Entity, World};

    use crate::testing;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let entity = world.spawn_empty().id();
        let spell = world.spawn_empty().id();
        let player = testing::TestPlayer(entity);

        assert_eq!(Entity::from(player), entity);
        assert_eq!(*player, entity);
        assert_eq!(player.cast(spell).steps().len(), 1);
    }
}
