use crate::HAND;
use crate::cards;
use crate::players;

pub fn hand(view: &players::PlayerView<'_, '_>) -> Vec<cards::CardId> {
    view.players
        .zone_contents
        .iter()
        .filter(|(zone, owner, _)| **zone == HAND && ***owner == view.id)
        .flat_map(|(_, _, contents)| contents.iter().copied().map(cards::CardId))
        .collect()
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::testing::*;

    use super::hand;
    use crate::players;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, _p2]) = MockGame::new([P1.hand([GRIZZLY_BEARS]), P2]);
        game.play([p1.pass_priority()])?;
        game.app
            .world_mut()
            .run_system_once(move |all: players::Players| {
                assert_eq!(hand(&all.view(p1.id)).len(), 1);
            })
            .unwrap();
        Ok(())
    }
}
