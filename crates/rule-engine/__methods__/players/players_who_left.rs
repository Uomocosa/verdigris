use crate::players;

pub fn players_who_left(players: &players::Players<'_, '_>) -> Vec<players::PlayerId> {
    players
        .in_turn_order()
        .into_iter()
        .filter(|player| !players.is_still_in_the_game(*player))
        .collect()
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::players;
    use crate::testing::*;

    use super::players_who_left;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2, _p3]) = MockGame::new([P1, P2.life(0), P3]);
        game.play([p1.pass_priority()])?;
        game.app
            .world_mut()
            .run_system_once(move |all: players::Players| {
                assert_eq!(players_who_left(&all), vec![p2.id]);
            })
            .unwrap();
        Ok(())
    }
}
