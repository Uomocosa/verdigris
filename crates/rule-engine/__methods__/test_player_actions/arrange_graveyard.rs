use bevy::prelude::Entity;

use crate::actions;
use crate::testing;

pub fn arrange_graveyard<const N: usize>(
    player_actions: testing::TestPlayerActions,
    cards: [testing::TestCard; N],
) -> testing::TestPlayerActions {
    player_actions.act(actions::Action::ArrangeGraveyard(
        cards.into_iter().map(Entity::from).collect(),
    ))
}

#[cfg(test)]
mod tests {
    use bevy::prelude::World;

    use crate::actions;
    use crate::testing;

    use super::arrange_graveyard;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let chooser = world.spawn_empty().id();
        let card = world.spawn_empty().id();

        assert_eq!(
            arrange_graveyard(
                testing::TestPlayerActions::of(testing::TestPlayer::from_entity_and_seat(
                    chooser,
                    testing::PlayerSeat(1)
                )),
                [testing::TestCard(card)]
            )
            .steps,
            vec![testing::PlayStep::Act(
                chooser,
                actions::Action::ArrangeGraveyard(vec![card])
            )]
        );
    }
}
