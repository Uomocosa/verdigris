use bevy::prelude::{MessageReader, MessageWriter, Res, ResMut};

use crate::turns;

/// 500.1. A turn consists of five phases, in this order: beginning, precombat main, combat,
/// postcombat main, and ending. Each of these phases takes place every turn, even if nothing
/// happens during the phase. The beginning, combat, and ending phases are further broken down into
/// steps, which proceed in order.
///
/// 500.2. A phase or step in which players receive priority ends when the stack is empty and all
/// players pass in succession. Simply having the stack become empty doesn’t cause such a phase or
/// step to end; all players have to pass in succession with the stack empty. Because of this, each
/// player gets a chance to add new things to the stack before that phase or step ends.
pub fn r500_1_r500_2_step_ends(
    mut all_players_passed: MessageReader<turns::AllPlayersPassed>,
    mut step_started: MessageWriter<turns::StepStarted>,
    mut turn: ResMut<turns::Turn>,
    mut priority: ResMut<turns::Priority>,
    combat_damage_steps: Res<turns::CombatDamageSteps>,
) {
    let combat_damage_steps = *combat_damage_steps.into_inner();
    for _ in all_players_passed.read() {
        let next = match turn.step {
            turns::Step::Untap => turns::Step::Upkeep,
            turns::Step::Upkeep => turns::Step::Draw,
            turns::Step::Draw => turns::Step::PrecombatMain,
            turns::Step::PrecombatMain => turns::Step::BeginningOfCombat,
            turns::Step::BeginningOfCombat => turns::Step::DeclareAttackers,
            turns::Step::DeclareAttackers | turns::Step::Cleanup => continue,
            turns::Step::DeclareBlockers => turns::Step::CombatDamage,
            turns::Step::CombatDamage => match combat_damage_steps {
                turns::CombatDamageSteps::FirstOfTwo => turns::Step::SecondCombatDamage,
                turns::CombatDamageSteps::Single | turns::CombatDamageSteps::SecondOfTwo => {
                    turns::Step::EndOfCombat
                }
            },
            turns::Step::SecondCombatDamage => turns::Step::EndOfCombat,
            turns::Step::EndOfCombat => turns::Step::PostcombatMain,
            turns::Step::PostcombatMain => turns::Step::End,
            turns::Step::End => turns::Step::Cleanup,
        };
        turn.step = next;
        priority.give_to(turn.active);
        step_started.write(turns::StepStarted(next));
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
        assert_eq!(priority.holder, p1.id);
        assert!(priority.passed.is_empty());
        Ok(())
    }
}
