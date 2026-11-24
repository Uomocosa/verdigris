use bevy::prelude::{Commands, MessageReader, ResMut};

use crate::Resolve;
use crate::StateBasedActionsPerformed;
use crate::players;
use crate::rules;
use crate::rules::traits::context::Context;

/// 104.2. There are several ways to win the game.
pub fn r104_2_win_the_game(
    mut proceeds: MessageReader<Resolve<rules::WinGame>>,
    mut commands: Commands,
    mut state_based_actions_performed: ResMut<StateBasedActionsPerformed>,
) {
    for happened in proceeds.read() {
        commands.entity(*happened.player()).insert(players::Won);
        **state_based_actions_performed = true;
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn laboratory_maniac_wins_during_the_draw_step_instead_of_drawing() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::builder()
            .turn(2, turns::Step::Upkeep)
            .players([P1.battlefield([LABORATORY_MANIAC]).empty_library(), P2]);
        game.play([p1.pass_priority(), p2.pass_priority(), p1.pass_priority()])?;
        assert!(game.has_won(p1));
        assert!(!game.has_lost(p1));
        Ok(())
    }
}
