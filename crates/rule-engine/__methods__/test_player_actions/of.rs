use crate::testing;

pub const fn of(player: testing::TestPlayer) -> testing::TestPlayerActions {
    testing::TestPlayerActions {
        player,
        steps: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::World;

    use crate::testing;

    use super::of;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let entity = world.spawn_empty().id();

        assert_eq!(
            of(testing::TestPlayer::from_entity_and_seat(
                entity,
                testing::PlayerSeat(1)
            ))
            .steps,
            Vec::new()
        );
    }
}
