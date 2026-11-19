use bevy::prelude::Entity;

use crate::testing;

pub const fn from_caster_and_spell(caster: Entity, spell: Entity) -> testing::Cast {
    testing::Cast {
        caster,
        spell,
        targets: Vec::new(),
        should_resolve: false,
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::World;

    use crate::testing::traits::Caster;

    use super::from_caster_and_spell;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let caster = world.spawn_empty().id();
        let spell = world.spawn_empty().id();

        let cast = from_caster_and_spell(caster, spell);

        assert_eq!(caster.cast(spell), cast);
        assert_eq!(cast.steps().len(), 1);
    }
}
