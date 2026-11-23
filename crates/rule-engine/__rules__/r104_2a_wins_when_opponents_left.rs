use bevy::prelude::{Entity, MessageWriter, Query, With};

use crate::players;
use crate::rules;

/// 104.2a A player still in the game wins the game if that player’s opponents have all left the game. This
/// happens immediately and overrides all effects that would preclude that player from winning the game.
pub fn r104_2a_wins_when_opponents_left(
    mut wins: MessageWriter<rules::Would<rules::WinGame>>,
    players: Query<(Entity, Option<&players::Lost>), With<players::Player>>,
    won: Query<Entity, With<players::Won>>,
) {
    let total = players.iter().count();
    if total < 2 {
        return;
    }
    let still_in_the_game: Vec<Entity> =
        players.iter().filter(|(_, lost)| lost.is_none()).map(|(player, _)| player).collect();
    let [winner] = still_in_the_game.as_slice() else {
        return;
    };
    if won.get(*winner).is_err() {
        wins.write(rules::Would(rules::WinGame(*winner)));
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;

    #[test]
    fn last_player_wins_when_their_opponent_has_left() -> TestResult {
        let (mut game, [p1, _p2]) = MockGame::new([P1, P2.life(0)]);
        game.play([p1.pass_priority()])?;
        assert!(game.has_won(p1));
        Ok(())
    }

    #[test]
    fn last_player_wins_when_every_opponent_has_left() -> TestResult {
        let (mut game, [p1, p2, p3]) = MockGame::new([P1, P2.life(0), P3.life(0)]);

        game.play([p1.pass_priority()])?;

        assert!(game.has_lost(p2));
        assert!(game.has_lost(p3));
        assert!(game.has_won(p1));
        assert!(!game.has_won(p2));
        assert!(!game.has_won(p3));
        Ok(())
    }

    #[test]
    fn no_winner_is_declared_while_an_opponent_is_still_in_the_game() -> TestResult {
        let (mut game, [p1, p2, p3]) = MockGame::new([P1, P2.life(0), P3]);

        game.play([p1.pass_priority()])?;

        assert!(game.has_lost(p2));
        assert!(!game.has_won(p1));
        assert!(!game.has_won(p3));
        Ok(())
    }
}
