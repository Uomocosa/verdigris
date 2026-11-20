use bevy::prelude::Entity;

use crate::actions;
use crate::testing;

pub fn discard<const N: usize>(
    player_actions: testing::TestPlayerActions,
    cards: [testing::TestCard; N],
) -> testing::TestPlayerActions {
    player_actions.act(actions::Action::ChooseDiscard(
        cards.into_iter().map(Entity::from).collect(),
    ))
}

#[cfg(test)]
mod tests {
    use bevy::prelude::World;

    use crate::actions;
    use crate::testing;

    use super::discard;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let caster = world.spawn_empty().id();
        let card = world.spawn_empty().id();

        assert_eq!(
            discard(
                testing::TestPlayerActions::of(testing::TestPlayer::from_entity_and_seat(
                    caster,
                    testing::PlayerSeat(1)
                )),
                [testing::TestCard(card)]
            )
            .steps,
            vec![testing::PlayStep::Act(
                caster,
                actions::Action::ChooseDiscard(vec![card])
            )]
        );
    }
}
