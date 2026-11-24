use bevy::prelude::{Commands, Entity, Query, ResMut, With};

use crate::StateBasedActionsPerformed;
use crate::players;

/// 104.4a If all the players remaining in a game lose simultaneously, the game is a draw.
pub fn r104_4a_draw(
    mut commands: Commands,
    mut state_based_actions_performed: ResMut<StateBasedActionsPerformed>,
    players: Query<(Entity, Option<&players::Lost>), With<players::Player>>,
    won: Query<Entity, With<players::Won>>,
    drawn: Query<Entity, With<players::GameIsADraw>>,
) {
    if !won.is_empty() || !drawn.is_empty() {
        return;
    }
    let remaining: Vec<Entity> = players.iter().map(|(player, _)| player).collect();
    if remaining.len() < 2 {
        return;
    }
    let do_all_remaining_players_lose = players.iter().all(|(_, lost)| lost.is_some());
    if !do_all_remaining_players_lose {
        return;
    }
    for player in remaining {
        commands.entity(player).insert(players::GameIsADraw);
    }
    **state_based_actions_performed = true;
}

#[cfg(test)]
mod tests {
    use crate::players;
    use crate::testing::*;

    #[test]
    fn simultaneous_loss_is_a_draw() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.life(0), P2.life(0)]);

        game.play([p1.pass_priority()])?;

        assert!(game.has_lost(p1));
        assert!(game.has_lost(p2));
        assert!(!game.has_won(p1));
        assert!(!game.has_won(p2));
        assert!(game.has::<players::GameIsADraw>(p1));
        assert!(game.has::<players::GameIsADraw>(p2));
        Ok(())
    }

    #[test]
    fn no_draw_while_a_player_survives() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.life(0), P2]);

        game.play([p1.pass_priority()])?;

        assert!(game.has_lost(p1));
        assert!(!game.has::<players::GameIsADraw>(p1));
        assert!(!game.has::<players::GameIsADraw>(p2));
        Ok(())
    }
}
