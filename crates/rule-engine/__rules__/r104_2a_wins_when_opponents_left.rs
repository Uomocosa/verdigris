use bevy::prelude::{Commands, Entity, Query, ResMut, With};

use crate::StateBasedActionsPerformed;
use crate::players;

/// 104.2a A player still in the game wins the game if that player’s opponents have all left the game. This
/// happens immediately and overrides all effects that would preclude that player from winning the game.
pub fn r104_2a_wins_when_opponents_left(
    mut commands: Commands,
    mut state_based_actions_performed: ResMut<StateBasedActionsPerformed>,
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
        **state_based_actions_performed = true;
    }
}

#[cfg(test)]
mod tests {
    use crate::actions;
    use crate::testing::*;

    #[test]
    fn r104_2a_sole_survivor_wins_despite_cant_win_effect() {
        let (mut game, [p1, p2]) = MockGame::new([P1.battlefield([PLATINUM_ANGEL]), P2]);

        game.act(p1, actions::Action::Concede);

        assert!(game.has_lost(p1));
        assert!(game.has_won(p2));
    }

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
