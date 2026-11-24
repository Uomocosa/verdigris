use bevy::prelude::{Commands, ResMut};

use crate::StateBasedActionsPerformed;
use crate::players;

/// 104.4a If all the players remaining in a game lose simultaneously, the game is a draw.
pub fn r104_4a_draw(
    mut commands: Commands,
    mut state_based_actions_performed: ResMut<StateBasedActionsPerformed>,
    players: players::Players,
) {
    let remaining = players.still_in_the_game();
    if remaining.is_empty() || !remaining.iter().all(players::PlayerView::has_lost) {
        return;
    }
    for player in remaining {
        commands.entity(*player.id).insert(players::GameIsADraw);
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

    #[test]
    fn a_player_who_already_left_is_not_part_of_the_draw() {
        use crate::actions;
        let (mut game, [p1, p2, p3]) = MockGame::new([P1.life(0), P2.life(0), P3]);

        game.act(p3, actions::Action::Concede);

        assert!(game.has_lost(p3));
        assert!(!game.has::<players::GameIsADraw>(p3));
        assert!(game.has::<players::GameIsADraw>(p1));
        assert!(game.has::<players::GameIsADraw>(p2));
    }
}
