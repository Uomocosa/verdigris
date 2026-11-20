use crate::actions;
use crate::testing;

pub fn targeting(
    player_actions: testing::TestPlayerActions,
    targets: impl Into<testing::Targets>,
) -> testing::TestPlayerActions {
    player_actions.act(actions::Action::ChooseTargets(
        targets.into().into_entities(),
    ))
}

#[cfg(test)]
mod tests {
    use bevy::prelude::World;

    use crate::actions;
    use crate::testing;

    use super::targeting;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let caster = world.spawn_empty().id();
        let target = world.spawn_empty().id();

        assert_eq!(
            targeting(
                testing::TestPlayerActions::of(testing::TestPlayer::from_entity_and_seat(
                    caster,
                    testing::PlayerSeat(1)
                )),
                target
            )
            .steps,
            vec![testing::PlayStep::Act(
                caster,
                actions::Action::ChooseTargets(vec![target])
            )]
        );
    }
}
