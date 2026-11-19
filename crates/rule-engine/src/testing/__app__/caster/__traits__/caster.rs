use bevy::prelude::Entity;

use crate::testing;

pub trait Caster {
    fn cast(self, spell: Entity) -> testing::Cast;
}

#[rustfmt::skip]
impl Caster for Entity {
    fn cast(self, spell: Entity) -> testing::Cast { testing::Cast::from_caster_and_spell(self, spell) }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::World;

    use crate::testing::traits::Caster;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let caster = world.spawn_empty().id();
        let spell = world.spawn_empty().id();

        assert_eq!(caster.cast(spell).steps().len(), 1);
    }
}
