use bevy::prelude::{MessageWriter, ResMut};

use crate::GameMut;
use crate::turns;

/// 500.3. A step in which no players receive priority ends when all specified actions that take
/// place during that step are completed. The only such steps are the untap step (see rule 502) and
/// certain cleanup steps (see rule 514).
///
/// 503.1. The upkeep step has no turn-based actions. Once it begins, the active player gets
/// priority. (See rule 117, “Timing and Priority.”)
///
/// # See also
///
/// 502.4. No player receives priority during the untap step, so no spells can be cast or resolve
/// and no abilities can be activated or resolve. Any ability that triggers during this step will be
/// held until the next time a player would receive priority, which is usually during the upkeep
/// step. (See rule 503, “Upkeep Step.”)
///
/// 512.1. The ending phase consists of two steps: end and cleanup.
///
/// 514.3. Normally, no player receives priority during the cleanup step, so no spells can be cast
/// and no abilities can be activated. However, this rule is subject to the following exception:
pub fn r500_3_r503_1_step_without_priority_ends(
    mut step_started: MessageWriter<turns::StepStarted>,
    mut game: GameMut,
    mut reentry: ResMut<turns::CleanupReentry>,
) {
    match game.step() {
        turns::Step::Cleanup => {
            if reentry.should_repeat {
                reentry.should_repeat = false;
                reentry.is_priority_window = false;
                reentry.is_extra_cleanup = true;
                step_started.write(turns::StepStarted(turns::Step::Cleanup));
                return;
            }
            if reentry.is_extra_cleanup {
                reentry.is_extra_cleanup = false;
                return;
            }
            if reentry.is_priority_window {
                return;
            }
            if game.begin_next_turn().is_some() {
                step_started.write(turns::StepStarted(turns::Step::Untap));
            }
        }
        turns::Step::Untap => {
            game.begin_step(turns::Step::Upkeep);
            game.give_priority_to_active_player();
            step_started.write(turns::StepStarted(turns::Step::Upkeep));
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use crate::actions;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn cleanup_ends_the_turn_and_the_next_player_becomes_active() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1, P2]);

        game.play([p1.end_turn(), p2.end_turn()])?;

        let turn = *game.turn();
        let priority = game.priority();
        assert_eq!(turn.number, 2);
        assert_eq!(turn.active, p2.id);
        assert_ne!(turn.active, p1.id);
        assert_eq!(turn.step, turns::Step::Untap);
        assert_eq!(priority.holder, Some(p2.id));
        assert!(priority.passed.is_empty());

        game.app.update();

        let turn = *game.turn();
        let priority = game.priority();
        assert_eq!(turn.step, turns::Step::Upkeep);
        assert_eq!(priority.holder, Some(p2.id));
        Ok(())
    }

    #[test]
    fn a_player_who_left_does_not_begin_a_turn() -> TestResult {
        let (mut game, [p1, p2, p3]) = MockGame::new([P1, P2, P3]);

        game.act(p2, actions::Action::Concede);
        game.play([p1.end_turn(), p3.end_turn()])?;

        assert_eq!(game.turn().active, p3.id);
        Ok(())
    }
}
