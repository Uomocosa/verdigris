use crate::players;

pub fn still_in_the_game(players: &players::Players) -> Vec<players::PlayerId> {
    players
        .in_turn_order()
        .into_iter()
        .filter(|player| players.is_still_in_the_game(*player))
        .collect()
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::testing::*;

    use super::still_in_the_game;
    use crate::players;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, _p2, p3]) = MockGame::new([P1, P2.life(0), P3]);
        game.play([p1.pass_priority()])?;
        game.app
            .world_mut()
            .run_system_once(move |all: players::Players| {
                assert_eq!(still_in_the_game(&all), [p1.id, p3.id]);
            })
            .unwrap();
        Ok(())
    }
}
