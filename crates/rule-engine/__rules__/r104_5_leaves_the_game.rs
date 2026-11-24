use bevy::prelude::{Commands, Entity, Query, With, Without};

use crate::players;

/// 104.5. If a player loses the game, that player leaves the game. If the game is a draw for a
/// player, that player leaves the game. The multiplayer rules handle what happens when a player
/// leaves the game; see rule 800.4.
pub fn r104_5_leaves_the_game(
    mut commands: Commands,
    players: Query<
        Entity,
        (
            With<players::Player>,
            players::LostOrGameIsADraw,
            Without<players::LeftTheGame>,
        ),
    >,
) {
    for player in players.iter() {
        commands.entity(player).insert(players::LeftTheGame);
    }
}

#[cfg(test)]
mod tests {
    use crate::players;
    use crate::testing::*;

    #[test]
    fn a_player_who_loses_leaves_the_game() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.life(0), P2]);

        game.play([p1.pass_priority()])?;

        assert!(game.has_lost(p1));
        assert!(game.has::<players::LeftTheGame>(p1));
        assert!(!game.has::<players::LeftTheGame>(p2));
        Ok(())
    }
}
