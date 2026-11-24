use bevy::prelude::{Commands, MessageReader, ResMut};

use crate::StateBasedActionsPerformed;
use crate::players;
use crate::rules;
use crate::rules::traits::context::Context;

/// 104.3. There are several ways to lose the game.
pub fn r104_3_lose_the_game(
    mut proceeds: MessageReader<rules::Proceeds<rules::LoseGame>>,
    mut commands: Commands,
    mut state_based_actions_performed: ResMut<StateBasedActionsPerformed>,
) {
    for happened in proceeds.read() {
        commands.entity(*happened.player()).insert(players::Lost);
        **state_based_actions_performed = true;
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;

    #[test]
    fn a_player_at_zero_life_loses_the_game() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.life(0), P2]);
        game.play([p1.pass_priority()])?;
        assert!(game.has_lost(p1));
        assert!(!game.has_lost(p2));
        Ok(())
    }
}
