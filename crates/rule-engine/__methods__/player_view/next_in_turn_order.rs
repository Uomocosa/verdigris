use crate::players;

pub fn next_in_turn_order<'w, 's>(
    view: &players::PlayerView<'w, 's>,
) -> Option<players::PlayerView<'w, 's>> {
    let order = view.players.in_turn_order();
    let start = order.iter().position(|candidate| *candidate == view.id)?;
    for candidate in order.iter().cycle().skip(start.saturating_add(1)).take(order.len()) {
        if view.players.is_still_in_the_game(*candidate) {
            return Some(view.players.view(*candidate));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::testing::*;

    use super::next_in_turn_order;
    use crate::players;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, _p2, p3]) = MockGame::new([P1, P2.life(0), P3]);
        game.play([p1.pass_priority()])?;
        game.app
            .world_mut()
            .run_system_once(move |all: players::Players| {
                assert_eq!(
                    next_in_turn_order(&all.view(p1.id)).map(|player| player.id),
                    Some(p3.id)
                );
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn wraps_around_the_end_of_turn_order() -> TestResult {
        let (mut game, [p1, _p2, p3]) = MockGame::new([P1, P2.life(0), P3]);
        game.play([p1.pass_priority()])?;
        game.app
            .world_mut()
            .run_system_once(move |all: players::Players| {
                assert_eq!(
                    next_in_turn_order(&all.view(p3.id)).map(|player| player.id),
                    Some(p1.id)
                );
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn starts_from_a_player_who_left() -> TestResult {
        let (mut game, [p1, p2, p3]) = MockGame::new([P1, P2.life(0), P3]);
        game.play([p1.pass_priority()])?;
        game.app
            .world_mut()
            .run_system_once(move |all: players::Players| {
                assert_eq!(
                    next_in_turn_order(&all.view(p2.id)).map(|player| player.id),
                    Some(p3.id)
                );
            })
            .unwrap();
        Ok(())
    }
}
