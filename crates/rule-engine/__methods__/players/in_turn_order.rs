use crate::players;

pub fn in_turn_order(players: &players::Players) -> Vec<players::PlayerId> {
    players
        .turn_order
        .iter()
        .flat_map(|order| order.iter().map(|entity| players::PlayerId(*entity)))
        .collect()
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::players;
    use crate::testing::*;

    use super::in_turn_order;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2, p3]) = MockGame::new([P1, P2, P3]);
        game.play([p1.pass_priority()])?;
        game.app
            .world_mut()
            .run_system_once(move |all: players::Players| {
                assert_eq!(in_turn_order(&all), [p1.id, p2.id, p3.id]);
            })
            .unwrap();
        Ok(())
    }
}
