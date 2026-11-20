use bevy::prelude::Entity;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerRef {
    You,
    Opponent,
    Any,
}

impl PlayerRef {
    #[must_use]
    pub fn resolve(self, controller: Entity, players: &[Entity]) -> Vec<Entity> {
        match self {
            Self::You => vec![controller],
            Self::Opponent => Self::players_other_than(players, controller),
            Self::Any => players.to_vec(),
        }
    }
}

#[rustfmt::skip]
impl PlayerRef {
    #[must_use]
    pub(crate) fn players_other_than(players: &[Entity], controller: Entity) -> Vec<Entity> { players.iter().copied().filter(|player| *player != controller).collect() }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{Entity, World};

    use super::PlayerRef;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let players: Vec<Entity> = (0..4).map(|_| world.spawn_empty().id()).collect();
        let controller = players[0];
        assert_eq!(
            PlayerRef::You.resolve(controller, &players),
            vec![controller]
        );
        assert_eq!(PlayerRef::Opponent.resolve(controller, &players).len(), 3);
        assert_eq!(PlayerRef::Any.resolve(controller, &players), players);
    }
}
