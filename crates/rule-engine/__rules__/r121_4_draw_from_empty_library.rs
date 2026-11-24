use bevy::prelude::{Commands, MessageReader, ResMut};

use crate::StateBasedActionsPerformed;
use crate::players;
use crate::rules;

/// 121.4. A player who attempts to draw a card from a library with no cards in it loses the game the next time a player would receive priority. (This is a state-based action. See rule 704.)
pub fn r121_4_draw_from_empty_library(
    mut proceeds: MessageReader<rules::Proceeds<rules::DrawFromEmptyLibrary>>,
    mut commands: Commands,
    mut state_based_actions_performed: ResMut<StateBasedActionsPerformed>,
) {
    for happened in proceeds.read() {
        commands.entity(****happened).insert(players::AttemptedDrawFromEmptyLibrary);
        **state_based_actions_performed = true;
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn drawing_from_an_empty_library_during_the_draw_step_attempts_the_draw() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::builder().turn(2, turns::Step::Upkeep).players([P1.empty_library(), P2]);
        game.play([p1.pass_priority(), p2.pass_priority(), p1.pass_priority()])?;
        assert!(game.has_lost(p1));
        assert!(!game.has_lost(p2));
        Ok(())
    }
}
