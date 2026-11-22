use bevy::prelude::{Entity, MessageReader, Query, ResMut, With};

use crate::cards;
use crate::keywords;
use crate::turns;

/// 506.1. The combat phase has five steps, which proceed in order: beginning of combat, declare
/// attackers, declare blockers, combat damage, and end of combat. The declare blockers and combat
/// damage steps are skipped if no creatures are declared as attackers or put onto the battlefield
/// attacking (see rule 508.8). There are two combat damage steps if any attacking or blocking
/// creature has first strike (see rule 702.7) or double strike (see rule 702.4).
///
/// 510.4. If at least one attacking or blocking creature has first strike (see rule 702.7) or
/// double strike (see rule 702.4) as the combat damage step begins, the only creatures that assign
/// combat damage in that step are those with first strike or double strike. After that step,
/// instead of proceeding to the end of combat step, the phase gets a second combat damage step.
/// The only creatures that assign combat damage in that step are the remaining attackers and
/// blockers that had neither first strike nor double strike as the first combat damage step began,
/// as well as the remaining attackers and blockers that currently have double strike. After that
/// step, the phase proceeds to the end of combat step.
///
/// 702.4b If at least one attacking or blocking creature has first strike (see rule 702.7) or
/// double strike as the combat damage step begins, the only creatures that assign combat damage in
/// that step are those with first strike or double strike. After that step, instead of proceeding
/// to the end of combat step, the phase gets a second combat damage step. The only creatures that
/// assign combat damage in that step are the remaining attackers and blockers that had neither
/// first strike nor double strike as the first combat damage step began, as well as the remaining
/// attackers and blockers that currently have double strike. After that step, the phase proceeds
/// to the end of combat step.
///
/// 702.7b If at least one attacking or blocking creature has first strike or double strike (see
/// rule 702.4) as the combat damage step begins, the only creatures that assign combat damage in
/// that step are those with first strike or double strike. After that step, instead of proceeding
/// to the end of combat step, the phase gets a second combat damage step. The only creatures that
/// assign combat damage in that step are the remaining attackers and blockers that had neither
/// first strike nor double strike as the first combat damage step began, as well as the remaining
/// attackers and blockers that currently have double strike. After that step, the phase proceeds
/// to the end of combat step.
pub fn r506_1_r510_4_r702_4b_r702_7b_combat_damage_steps(
    mut step_started: MessageReader<turns::StepStarted>,
    mut combat_damage_steps: ResMut<turns::CombatDamageSteps>,
    mut participants: ResMut<turns::CombatDamageParticipants>,
    attackers: Query<(Entity, &cards::CardDefinition), With<cards::Attacking>>,
    blockers: Query<(Entity, &cards::CardDefinition), With<cards::Blocking>>,
) {
    let has_first_strike = |definition: &cards::CardDefinition| {
        definition.has_keyword(keywords::StaticKeyword::FirstStrike)
    };
    let has_double_strike = |definition: &cards::CardDefinition| {
        definition.has_keyword(keywords::StaticKeyword::DoubleStrike)
    };
    let does_assign_in_first_step = |definition: &cards::CardDefinition| {
        has_first_strike(definition) || has_double_strike(definition)
    };
    let does_assign_in_second_step = |definition: &cards::CardDefinition| {
        !has_first_strike(definition) || has_double_strike(definition)
    };
    for step in step_started.read() {
        match **step {
            turns::Step::CombatDamage => {
                let does_a_striker_exist = attackers
                    .iter()
                    .chain(blockers.iter())
                    .any(|(_, definition)| does_assign_in_first_step(definition));
                *combat_damage_steps = if does_a_striker_exist {
                    turns::CombatDamageSteps::FirstOfTwo
                } else {
                    turns::CombatDamageSteps::Single
                };
                let does_assign = |definition: &cards::CardDefinition| {
                    !does_a_striker_exist || does_assign_in_first_step(definition)
                };
                **participants = attackers
                    .iter()
                    .filter(|(_, definition)| does_assign(definition))
                    .map(|(entity, _)| entity)
                    .chain(
                        blockers
                            .iter()
                            .filter(|(_, definition)| does_assign(definition))
                            .map(|(entity, _)| entity),
                    )
                    .collect();
            }
            turns::Step::SecondCombatDamage => {
                *combat_damage_steps = turns::CombatDamageSteps::SecondOfTwo;
                **participants = attackers
                    .iter()
                    .filter(|(_, definition)| does_assign_in_second_step(definition))
                    .map(|(entity, _)| entity)
                    .chain(
                        blockers
                            .iter()
                            .filter(|(_, definition)| does_assign_in_second_step(definition))
                            .map(|(entity, _)| entity),
                    )
                    .collect();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;
    use crate::turns;
    use crate::{BATTLEFIELD, GRAVEYARD};

    #[test]
    fn a_double_striker_deals_damage_in_both_combat_damage_steps() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([FENCING_ACE]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [ace] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(FENCING_ACE)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([ace]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
            p1.pass_priority(),
            p2.declare_blockers([(bears, ace)]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_priority(),
            p2.pass_priority(),
        ])?;

        assert_eq!(game.damage(bears), 1);
        assert!(game.player(p1).cards_in(BATTLEFIELD).has(ace));
        assert_eq!(game.turn().step, turns::Step::SecondCombatDamage);

        game.play([p1.pass_priority(), p2.pass_priority()])?;

        assert!(game.player(p1).cards_in(GRAVEYARD).has(ace));
        assert!(game.player(p2).cards_in(GRAVEYARD).has(bears));
        assert_eq!(game.turn().step, turns::Step::EndOfCombat);
        Ok(())
    }
}
