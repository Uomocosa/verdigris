use bevy::prelude::Entity;

use crate::actions;
use crate::testing;

pub fn declare_attackers<const N: usize>(
    player_actions: testing::TestPlayerActions,
    attackers: [testing::TestCard; N],
) -> testing::TestPlayerActions {
    player_actions.act(actions::Action::DeclareAttackers(
        attackers.into_iter().map(Entity::from).collect(),
    ))
}

#[cfg(test)]
mod tests {
    use bevy::prelude::World;

    use crate::actions;
    use crate::testing;

    use super::declare_attackers;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let caster = world.spawn_empty().id();
        let attacker = world.spawn_empty().id();

        assert_eq!(
            declare_attackers(
                testing::TestPlayerActions::of(testing::TestPlayer::from_entity_and_seat(
                    caster,
                    testing::PlayerSeat(1)
                )),
                [testing::TestCard(attacker)]
            )
            .steps,
            vec![testing::PlayStep::Act(
                caster,
                actions::Action::DeclareAttackers(vec![attacker])
            )]
        );
    }
}
