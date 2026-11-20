use bevy::prelude::{MessageWriter, Res, ResMut};

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
    mut turn: ResMut<turns::Turn>,
    mut priority: ResMut<turns::Priority>,
    turn_order: Res<turns::TurnOrder>,
) {
    let turn_order = turn_order.into_inner();
    match turn.step {
        turns::Step::Cleanup => {
            let Some(next) = turn_order.next_after(turn.active) else {
                return;
            };
            turn.number = turn.number.saturating_add(1);
            turn.active = next;
            turn.step = turns::Step::Untap;
            priority.consecutive_passes = 0;
            priority.player = next;
            step_started.write(turns::StepStarted(turns::Step::Untap));
        }
        turns::Step::Untap => {
            turn.step = turns::Step::Upkeep;
            priority.player = turn.active;
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1, P2]);

        game.play([p1.end_turn(), p2.end_turn()])?;

        assert_eq!(game.app.world().resource::<turns::Turn>().number, 2);
        Ok(())
    }

    #[test]
    fn cleanup_ends_the_turn_and_the_next_player_becomes_active() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1, P2]);

        game.play([p1.end_turn(), p2.end_turn()])?;

        let turn = *game.app.world().resource::<turns::Turn>();
        let priority = *game.app.world().resource::<turns::Priority>();
        assert_eq!(turn.number, 2);
        assert_eq!(turn.active, p2);
        assert_ne!(turn.active, p1);
        assert_eq!(turn.step, turns::Step::Untap);
        assert_eq!(priority.player, p2);
        assert_eq!(priority.consecutive_passes, 0);

        game.app.update();

        let turn = *game.app.world().resource::<turns::Turn>();
        let priority = *game.app.world().resource::<turns::Priority>();
        assert_eq!(turn.step, turns::Step::Upkeep);
        assert_eq!(priority.player, p2);
        Ok(())
    }
}
