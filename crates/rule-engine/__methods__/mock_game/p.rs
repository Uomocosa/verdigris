use bevy::prelude::Entity;

use crate::testing;

pub fn p(game: &testing::MockGame, seat: usize) -> Entity {
    let index = seat.saturating_sub(1);
    let player = game.players.get(index).copied();
    assert!(
        player.is_some(),
        "MockGame: P{seat} does not exist; the game has {} players",
        game.players.len()
    );
    player.unwrap_or(Entity::PLACEHOLDER)
}

#[cfg(test)]
mod tests {
    use crate::testing::*;

    use super::p;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::new([P1, P2]);
        assert_eq!(p(&game, 1), p1);
    }

    #[test]
    #[should_panic(expected = "MockGame: P3 does not exist")]
    fn p_rejects_a_seat_the_game_does_not_have() {
        let (game, _) = MockGame::new([P1, P2]);
        let _ = p(&game, 3);
    }
}
