use bevy::prelude::Entity;

use crate::testing;

pub const fn from_caster_and_spell(caster: Entity, spell: Entity) -> testing::TestPlayerActions {
    testing::TestPlayerActions {
        caster,
        spell,
        targets: Vec::new(),
        should_resolve: false,
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::World;

    use crate::testing;

    use super::from_caster_and_spell;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let caster = world.spawn_empty().id();
        let spell = world.spawn_empty().id();

        let cast = from_caster_and_spell(caster, spell);

        assert_eq!(testing::TestPlayer(caster).cast(spell), cast);
        assert_eq!(cast.steps().len(), 1);
    }
}
