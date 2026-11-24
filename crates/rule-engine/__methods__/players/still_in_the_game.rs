use bevy::prelude::Entity;

use crate::players;

pub fn still_in_the_game(players: &players::Players) -> Vec<Entity> {
    players
        .in_turn_order()
        .into_iter()
        .filter(|player| players.is_still_in_the_game(*player))
        .collect()
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::Entity;

    use crate::testing::*;

    use super::still_in_the_game;
    use crate::players;

    #[test]
    fn test_usage() {
        let (mut game, [p1, p2]) = MockGame::new([P1, P2]);
        game.app
            .world_mut()
            .run_system_once(move |all: players::Players| {
                assert_eq!(
                    still_in_the_game(&all),
                    [Entity::from(p1), Entity::from(p2)]
                );
            })
            .unwrap();
    }
}
