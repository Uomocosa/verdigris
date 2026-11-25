use bevy::prelude::MessageReader;

use crate::turns;

pub fn begins(step: turns::Step) -> impl FnMut(MessageReader<turns::StepStarted>) -> bool + Clone {
    move |mut started: MessageReader<turns::StepStarted>| {
        started.read().filter(|started| ***started == step).count() > 0
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::MessageReader;

    use crate::testing::*;
    use crate::turns;

    use super::begins;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::builder().turn(2, turns::Step::Upkeep).players([P1, P2]);
        game.play([p1.pass_priority(), p2.pass_priority()])?;
        let draw_begins = game
            .app
            .world_mut()
            .run_system_once(|started: MessageReader<turns::StepStarted>| {
                begins(turns::Step::Draw)(started)
            })
            .unwrap();
        assert!(draw_begins);
        let upkeep_begins = game
            .app
            .world_mut()
            .run_system_once(|started: MessageReader<turns::StepStarted>| {
                begins(turns::Step::Upkeep)(started)
            })
            .unwrap();
        assert!(!upkeep_begins);
        Ok(())
    }
}
