use bevy::prelude::{Entity, Resource};
use derive_more::{Deref, From};

#[derive(Resource, Debug, Clone, PartialEq, Eq, Deref, From)]
pub struct TurnOrder(Vec<Entity>);

impl TurnOrder {
    #[must_use]
    pub fn next_after(&self, player: Entity) -> Option<Entity> {
        let position = self.iter().position(|candidate| *candidate == player)?;
        self.iter().cycle().nth(position.saturating_add(1)).copied()
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::World;

    use super::TurnOrder;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let first = world.spawn_empty().id();
        let second = world.spawn_empty().id();
        let third = world.spawn_empty().id();
        let fourth = world.spawn_empty().id();
        let outsider = world.spawn_empty().id();
        let order = TurnOrder::from(vec![first, second, third, fourth]);

        assert_eq!(order.next_after(first), Some(second));
        assert_eq!(order.next_after(fourth), Some(first));
        assert_eq!(order.next_after(outsider), None);
    }
}
