use bevy::prelude::Entity;

use crate::actions;
use crate::testing;

pub fn order_triggers<const N: usize>(
    player_actions: testing::TestPlayerActions,
    triggers: [testing::TestCard; N],
) -> testing::TestPlayerActions {
    player_actions.act(actions::Action::OrderTriggers(
        triggers.into_iter().map(Entity::from).collect(),
    ))
}

#[cfg(test)]
mod tests {
    use bevy::prelude::World;

    use crate::actions;
    use crate::testing;

    use super::order_triggers;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let chooser = world.spawn_empty().id();
        let trigger = world.spawn_empty().id();

        assert_eq!(
            order_triggers(
                testing::TestPlayerActions::of(testing::TestPlayer::from_entity_and_seat(
                    chooser,
                    testing::PlayerSeat(1)
                )),
                [testing::TestCard(trigger)]
            )
            .steps,
            vec![testing::PlayStep::Act(
                chooser,
                actions::Action::OrderTriggers(vec![trigger])
            )]
        );
    }
}
