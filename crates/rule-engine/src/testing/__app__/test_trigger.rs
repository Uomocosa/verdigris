use bevy::prelude::Entity;
use derive_more::Deref;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deref)]
pub struct TestTrigger(#[deref] pub(crate) Entity);

#[rustfmt::skip]
impl From<TestTrigger> for Entity {
    fn from(trigger: TestTrigger) -> Self { *trigger }
}

#[rustfmt::skip]
impl PartialEq<TestTrigger> for Entity {
    fn eq(&self, other: &TestTrigger) -> bool { *self == **other }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{Entity, World};

    use crate::testing;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let entity = world.spawn_empty().id();
        let trigger = testing::TestTrigger(entity);

        assert_eq!(Entity::from(trigger), entity);
        assert_eq!(*trigger, entity);
    }
}
