use crate::actions;
use crate::testing;

pub fn declare_blockers<const N: usize>(
    player_actions: testing::TestPlayerActions,
    blocks: [(testing::TestCard, testing::TestCard); N],
) -> testing::TestPlayerActions {
    player_actions.act(actions::Action::DeclareBlockers(
        blocks
            .into_iter()
            .map(|(blocker, attacker)| actions::BlockAssignment {
                blocker: blocker.into(),
                attacker: attacker.into(),
            })
            .collect(),
    ))
}

#[cfg(test)]
mod tests {
    use bevy::prelude::World;

    use crate::actions;
    use crate::testing;

    use super::declare_blockers;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let caster = world.spawn_empty().id();
        let blocker = world.spawn_empty().id();
        let attacker = world.spawn_empty().id();

        assert_eq!(
            declare_blockers(
                testing::TestPlayerActions::of(testing::TestPlayer::from_entity_and_seat(
                    caster,
                    testing::PlayerSeat(1)
                )),
                [(testing::TestCard(blocker), testing::TestCard(attacker))]
            )
            .steps,
            vec![testing::PlayStep::Act(
                caster,
                actions::Action::DeclareBlockers(vec![actions::BlockAssignment {
                    blocker,
                    attacker,
                }])
            )]
        );
    }
}
