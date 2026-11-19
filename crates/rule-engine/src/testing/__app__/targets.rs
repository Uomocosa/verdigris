use bevy::prelude::Entity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Targets {
    One(Entity),
    Many(Vec<Entity>),
}

#[rustfmt::skip]
impl From<Entity> for Targets {
    fn from(entity: Entity) -> Self { Self::One(entity) }
}

#[rustfmt::skip]
impl<const N: usize> From<[Entity; N]> for Targets {
    fn from(entities: [Entity; N]) -> Self { Self::Many(entities.into()) }
}

#[rustfmt::skip]
impl From<Vec<Entity>> for Targets {
    fn from(entities: Vec<Entity>) -> Self { Self::Many(entities) }
}

impl Targets {
    #[must_use]
    pub fn into_entities(self) -> Vec<Entity> {
        match self {
            Self::One(entity) => vec![entity],
            Self::Many(entities) => entities,
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::World;

    use super::Targets;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let first = world.spawn_empty().id();
        let second = world.spawn_empty().id();

        assert_eq!(Targets::from(first).into_entities(), vec![first]);
        assert_eq!(
            Targets::from([first, second]).into_entities(),
            vec![first, second]
        );
        assert_eq!(
            Targets::from(vec![first, second]).into_entities(),
            vec![first, second]
        );
    }
}
