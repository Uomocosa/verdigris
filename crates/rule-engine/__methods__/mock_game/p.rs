use bevy::prelude::Entity;

use crate::players;
use crate::testing;

pub fn p(game: &testing::MockGame, seat: usize) -> testing::TestPlayer {
    let player_seat = testing::PlayerSeat(seat);
    let index = seat.saturating_sub(1);
    let player = game.players.get(index).copied();
    assert!(
        player.is_some(),
        "MockGame: {player_seat} does not exist; the game has {} players",
        game.players.len()
    );
    testing::TestPlayer::from_entity_and_seat(
        *player.unwrap_or(players::PlayerId(Entity::PLACEHOLDER)),
        player_seat,
    )
}

#[cfg(test)]
mod tests {
    use crate::testing::*;

    use super::p;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::new([P1, P2]);
        assert_eq!(p(&game, 1), p1);
        assert_eq!(*p(&game, 1).seat, 1);
    }

    #[test]
    #[should_panic(expected = "MockGame: P3 does not exist")]
    fn p_rejects_a_seat_the_game_does_not_have() {
        let (game, _) = MockGame::new([P1, P2]);
        let _ = p(&game, 3);
    }
}
