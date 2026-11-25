use bevy::prelude::{MessageMutator, Query, With};

use crate::cards;
use crate::turns;

/// 508.8. If no creatures are declared as attackers or put onto the battlefield attacking, skip
/// the declare blockers and combat damage steps.
pub fn r508_8_skip_combat_steps(
    mut next_step: MessageMutator<turns::NextStep>,
    attackers: Query<(), With<cards::Attacking>>,
) {
    for step in next_step.read() {
        if **step == turns::Step::DeclareBlockers && attackers.is_empty() {
            *step = turns::NextStep(turns::Step::EndOfCombat);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn no_attackers_skips_the_declare_blockers_and_combat_damage_steps() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1, P2]);

        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])?;
        game.play([p1.pass_priority(), p2.pass_priority()])?;

        assert_eq!(game.turn().step, turns::Step::EndOfCombat);
        Ok(())
    }

    #[test]
    fn attackers_continue_to_declare_blockers() -> TestResult {
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

        assert_eq!(game.turn().step, turns::Step::DeclareBlockers);
        Ok(())
    }
}
