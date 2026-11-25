use bevy::prelude::{MessageReader, MessageWriter, Res};

use crate::Game;
use crate::turns;

/// 500.2. A phase or step in which players receive priority ends when the stack is empty and all
/// players pass in succession. Simply having the stack become empty doesn’t cause such a phase or
/// step to end; all players have to pass in succession with the stack empty. Because of this, each
/// player gets a chance to add new things to the stack before that phase or step ends.
pub fn r500_2_step_ends(
    mut all_players_passed: MessageReader<turns::AllPlayersPassed>,
    mut next_step: MessageWriter<turns::NextStep>,
    game: Game,
    combat_damage_steps: Res<turns::CombatDamageSteps>,
) {
    let combat_damage_steps = *combat_damage_steps.into_inner();
    for _ in all_players_passed.read() {
        let Some(current) = game.step() else { continue };
        let next = match current {
            turns::Step::Untap => turns::Step::Upkeep,
            turns::Step::Upkeep => turns::Step::Draw,
            turns::Step::Draw => turns::Step::PrecombatMain,
            turns::Step::PrecombatMain => turns::Step::BeginningOfCombat,
            turns::Step::BeginningOfCombat => turns::Step::DeclareAttackers,
            turns::Step::DeclareAttackers => turns::Step::DeclareBlockers,
            turns::Step::Cleanup => continue,
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
