use bevy::prelude::{MessageReader, MessageWriter, Res};

use crate::GameMut;
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
    mut game: GameMut,
    combat_damage_steps: Res<turns::CombatDamageSteps>,
) {
    let combat_damage_steps = *combat_damage_steps.into_inner();
    for _ in all_players_passed.read() {
        let next = match game.step() {
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
        game.begin_step(next);
        game.give_priority_to_active_player();
        step_started.write(turns::StepStarted(next));
    }
}

#[cfg(test)]
mod tests {
    use crate::actions;
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

    #[test]
    fn priority_goes_to_the_next_player_when_the_active_player_left() -> TestResult {
        let (mut game, [p1, p2, p3]) = MockGame::new([P1, P2, P3]);

        game.play([p1.pass_priority()])?;
        game.act(p1, actions::Action::Concede);
        game.play([p2.pass_priority(), p3.pass_priority()])?;

        assert_eq!(game.turn().step, turns::Step::BeginningOfCombat);
        assert_eq!(game.priority_player(), p2.id);
        Ok(())
    }
}
