use bevy::ecs::schedule::{IntoScheduleConfigs, ScheduleConfigs};
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::{MessageWriter, on_message};

use crate::Game;
use crate::turns;

/// 500.2. A phase or step in which players receive priority ends when the stack is empty and all
/// players pass in succession. Simply having the stack become empty doesn’t cause such a phase or
/// step to end; all players have to pass in succession with the stack empty. Because of this, each
/// player gets a chance to add new things to the stack before that phase or step ends.
pub fn r500_2_step_ends() -> ScheduleConfigs<ScheduleSystem> {
    step_ends.run_if(on_message::<turns::AllPlayersPassed>)
}

fn step_ends(mut next_step: MessageWriter<turns::NextStep>, game: Game) {
    if let Some(next) = game.turn().next_step() {
        next_step.write(turns::NextStep(next));
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn all_players_passing_with_an_empty_stack_ends_the_step() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1, P2]);

        game.play([p1.pass_priority(), p2.pass_priority()])?;

        let priority = game.priority();
        let turn = *game.turn();
        assert_eq!(turn.step, turns::Step::BeginningOfCombat);
        assert_eq!(priority.holder, Some(p1.id));
        assert!(priority.passed.is_empty());
        Ok(())
    }
}
