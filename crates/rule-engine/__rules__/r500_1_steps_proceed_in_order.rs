use bevy::prelude::{MessageReader, MessageWriter};

use crate::GameMut;
use crate::turns;

/// 500.1. A turn consists of five phases, in this order: beginning, precombat main, combat,
/// postcombat main, and ending. Each of these phases takes place every turn, even if nothing
/// happens during the phase. The beginning, combat, and ending phases are further broken down into
/// steps, which proceed in order.
pub fn r500_1_steps_proceed_in_order(
    mut next_step: MessageReader<turns::NextStep>,
    mut step_started: MessageWriter<turns::StepStarted>,
    mut game: GameMut,
) {
    for next in next_step.read() {
        game.begin_step(**next);
        game.give_priority_to_active_player();
        step_started.write(turns::StepStarted(**next));
    }
}

#[cfg(test)]
mod tests {
    use crate::actions;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn priority_goes_to_the_next_player_when_the_active_player_left() -> TestResult {
        let (mut game, [p1, p2, p3]) = MockGame::new([P1, P2, P3]);

        game.play([p1.pass_priority()])?;
        game.act(p1, actions::Action::Concede);
        game.play([p2.pass_priority(), p3.pass_priority()])?;

        assert_eq!(game.turn().step, turns::Step::BeginningOfCombat);
        assert_eq!(game.priority_player(), Some(p2.id));
        Ok(())
    }
}
