use crate::players;

pub fn opponents<'w, 's>(view: &players::PlayerView<'w, 's>) -> Vec<players::PlayerView<'w, 's>> {
    view.players
        .in_turn_order()
        .into_iter()
        .filter(|other| *other != view.id)
        .map(|id| view.players.view(id))
        .collect()
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::testing::*;

    use super::opponents;
    use crate::players;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2, p3]) = MockGame::new([P1, P2, P3.life(0)]);
        game.play([p1.pass_priority()])?;
        game.app
            .world_mut()
            .run_system_once(move |all: players::Players| {
                assert_eq!(
                    opponents(&all.view(p1.id))
                        .into_iter()
                        .map(|player| player.id)
                        .collect::<Vec<_>>(),
                    [p2.id, p3.id]
                );
            })
            .unwrap();
        Ok(())
    }
}
