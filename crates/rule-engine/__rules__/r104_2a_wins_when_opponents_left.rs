use bevy::prelude::{Commands, Entity, Query, With};

use crate::players;

/// 104.2a A player still in the game wins the game if that player’s opponents have all left the game. This
/// happens immediately and overrides all effects that would preclude that player from winning the game.
pub fn r104_2a_wins_when_opponents_left(
    mut commands: Commands,
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
        commands.entity(*winner).insert(players::Won);
    }
}

#[cfg(test)]
mod tests {
    use crate::players;
    use crate::testing::*;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, _p2]) = MockGame::new([P1, P2.life(0)]);
        game.play([p1.pass_priority()])?;
        assert!(game.app.world().get::<players::Won>(*p1).is_some());
        Ok(())
    }

    #[test]
    fn r104_2a_the_last_player_in_the_game_wins_when_every_opponent_has_left() -> TestResult {
        let (mut game, [p1, p2, p3]) = MockGame::new([P1, P2.life(0), P3.life(0)]);

        game.play([p1.pass_priority()])?;

        assert!(game.app.world().get::<players::Lost>(*p2).is_some());
        assert!(game.app.world().get::<players::Lost>(*p3).is_some());
        assert!(game.app.world().get::<players::Won>(*p1).is_some());
        assert!(game.app.world().get::<players::Won>(*p2).is_none());
        assert!(game.app.world().get::<players::Won>(*p3).is_none());
        Ok(())
    }

    #[test]
    fn r104_2a_no_winner_is_declared_while_an_opponent_is_still_in_the_game() -> TestResult {
        let (mut game, [p1, p2, p3]) = MockGame::new([P1, P2.life(0), P3]);

        game.play([p1.pass_priority()])?;

        assert!(game.app.world().get::<players::Lost>(*p2).is_some());
        assert!(game.app.world().get::<players::Won>(*p1).is_none());
        assert!(game.app.world().get::<players::Won>(*p3).is_none());
        Ok(())
    }
}
