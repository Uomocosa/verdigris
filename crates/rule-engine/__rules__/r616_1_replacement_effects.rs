use bevy::prelude::{Commands, MessageReader, ResMut};

use crate::StateBasedActionsPerformed;
use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;

/// 616.1. If two or more replacement and/or prevention effects are attempting to modify the way an
/// event affects an object or player, the affected object’s controller (or its owner if it has no
/// controller) or the affected player chooses one to apply, following the steps listed below. If
/// two or more players have to make these choices at the same time, choices are made in APNAP
/// order (see rule 101.4).
pub fn r616_1_replacement_effects(
    mut losses: MessageReader<rules::Would<rules::LoseGame>>,
    mut wins: MessageReader<rules::Would<rules::WinGame>>,
    mut commands: Commands,
    mut state_based_actions_performed: ResMut<StateBasedActionsPerformed>,
    answers: rules::Rules,
) {
    for would in losses.read() {
        let rules::Would(event) = would;
        if answers.legal::<rules::LosesTheGame>(&event.context()) {
            commands.entity(**event).insert(players::Lost);
            **state_based_actions_performed = true;
        }
    }
    for would in wins.read() {
        let rules::Would(event) = would;
        if answers.legal::<rules::WinsTheGame>(&event.context()) {
            commands.entity(**event).insert(players::Won);
            **state_based_actions_performed = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn opponent_cannot_win_while_platinum_angel_is_on_the_battlefield() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::builder().turn(1, turns::Step::Untap).players([
            P1.battlefield([FELIDAR_SOVEREIGN]).life(40),
            P2.battlefield([PLATINUM_ANGEL]),
        ]);
        game.play([p1.pass_priority(), p1.pass_priority(), p2.pass_priority()])?;
        assert!(!game.has_won(p1));
        assert!(!game.has_lost(p1));
        Ok(())
    }
}
