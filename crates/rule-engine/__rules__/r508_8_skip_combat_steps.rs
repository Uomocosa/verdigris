use bevy::prelude::{MessageMutator, MessageReader, Query, ResMut, With};

use crate::cards;
use crate::turns;

/// 508.8. If no creatures are declared as attackers or put onto the battlefield attacking, skip
/// the declare blockers and combat damage steps.
pub fn r508_8_skip_combat_steps(
    mut all_players_passed: MessageReader<turns::AllPlayersPassed>,
    mut step_started: MessageMutator<turns::StepStarted>,
    mut turn: ResMut<turns::Turn>,
    mut priority: ResMut<turns::Priority>,
    attackers: Query<(), With<cards::Attacking>>,
) {
    if all_players_passed.is_empty() {
        return;
    }
    all_players_passed.clear();
    if turn.step != turns::Step::DeclareAttackers {
        return;
    }
    for step in step_started.read() {
        if **step == turns::Step::DeclareAttackers {
            return;
        }
    }
    let next = if attackers.is_empty() {
        turns::Step::EndOfCombat
    } else {
        turns::Step::DeclareBlockers
    };
    turn.step = next;
    priority.consecutive_passes = 0;
    priority.player = turn.active;
    step_started.write(turns::StepStarted(next));
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn r508_8_no_attackers_skips_the_declare_blockers_and_combat_damage_steps() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1, P2]);

        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])?;
        game.play([p1.pass_priority(), p2.pass_priority()])?;

        assert_eq!(
            game.app.world().resource::<turns::Turn>().step,
            turns::Step::EndOfCombat
        );
        Ok(())
    }

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([bears]),
        ])?;
        game.play([p1.pass_priority(), p2.pass_priority()])?;

        assert_eq!(
            game.app.world().resource::<turns::Turn>().step,
            turns::Step::DeclareBlockers
        );
        Ok(())
    }
}
