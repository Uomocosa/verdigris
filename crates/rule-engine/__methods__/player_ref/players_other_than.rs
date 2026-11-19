use bevy::prelude::Entity;

pub fn players_other_than(players: &[Entity], controller: Entity) -> Vec<Entity> {
    players
        .iter()
        .copied()
        .filter(|player| *player != controller)
        .collect()
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{Entity, World};

    use super::players_other_than;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let players: Vec<Entity> = (0..4).map(|_| world.spawn_empty().id()).collect();
        let others = players_other_than(&players, players[0]);
        assert_eq!(others.len(), 3);
        assert!(!others.contains(&players[0]));
    }
}
