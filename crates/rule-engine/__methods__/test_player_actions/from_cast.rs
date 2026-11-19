use bevy::prelude::Entity;

use crate::actions;
use crate::testing;

pub fn from_cast(player: testing::TestPlayer, spell: Entity) -> testing::TestPlayerActions {
    testing::TestPlayerActions {
        player,
        steps: vec![testing::PlayStep::Act(
            *player,
            actions::Action::Cast(spell),
        )],
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::World;

    use crate::testing;

    use super::from_cast;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let caster = world.spawn_empty().id();
        let spell = world.spawn_empty().id();

        assert_eq!(from_cast(testing::TestPlayer(caster), spell).steps.len(), 1);
    }
}
