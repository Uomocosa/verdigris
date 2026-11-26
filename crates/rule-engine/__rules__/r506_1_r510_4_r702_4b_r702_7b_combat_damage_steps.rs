use bevy::prelude::{Entity, Has, MessageReader, Query, ResMut, With};

use crate::GameMut;
use crate::cards;
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
    mut game: GameMut,
    mut participants: ResMut<turns::CombatDamageParticipants>,
    attackers: Query<
        (Entity, Has<cards::FirstStrike>, Has<cards::DoubleStrike>),
        With<cards::Attacking>,
    >,
    blockers: Query<
        (Entity, Has<cards::FirstStrike>, Has<cards::DoubleStrike>),
        With<cards::Blocking>,
    >,
) {
    let does_assign_in_first_step =
        |first_strike: bool, double_strike: bool| first_strike || double_strike;
    let does_assign_in_second_step =
        |first_strike: bool, double_strike: bool| !first_strike || double_strike;
    for step in step_started.read() {
        match **step {
            turns::Step::CombatDamage => {
                let does_any_have_first_or_double_strike = attackers
                    .iter()
                    .chain(blockers.iter())
                    .any(|(_, first_strike, double_strike)| {
                        does_assign_in_first_step(first_strike, double_strike)
                    });
                if does_any_have_first_or_double_strike {
                    game.add_step_after_this_one(turns::Step::SecondCombatDamage);
                }
                let does_assign = |first_strike: bool, double_strike: bool| {
                    !does_any_have_first_or_double_strike
                        || does_assign_in_first_step(first_strike, double_strike)
                };
                **participants = attackers
                    .iter()
                    .filter(|(_, first_strike, double_strike)| {
                        does_assign(*first_strike, *double_strike)
                    })
                    .map(|(entity, _, _)| entity)
                    .chain(
                        blockers
                            .iter()
                            .filter(|(_, first_strike, double_strike)| {
                                does_assign(*first_strike, *double_strike)
                            })
                            .map(|(entity, _, _)| entity),
                    )
                    .collect();
            }
            turns::Step::SecondCombatDamage => {
                **participants = attackers
                    .iter()
                    .filter(|(_, first_strike, double_strike)| {
                        does_assign_in_second_step(*first_strike, *double_strike)
                    })
                    .map(|(entity, _, _)| entity)
                    .chain(
                        blockers
                            .iter()
                            .filter(|(_, first_strike, double_strike)| {
                                does_assign_in_second_step(*first_strike, *double_strike)
                            })
                            .map(|(entity, _, _)| entity),
                    )
                    .collect();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::mana;
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

    #[test]
    fn a_creature_given_first_strike_by_an_instant_deals_damage_only_in_the_first_step()
    -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS, MOUNTAIN])
                .hand([TEST_FIRST_STRIKE_GRANT])
                .mana_pool(mana![R]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [attacker] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [mountain] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(MOUNTAIN)?;
        let [blocker] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [grant] = game.player(p1).cards_in(HAND).exactly::<1>(TEST_FIRST_STRIKE_GRANT)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([attacker]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
            p1.pass_priority(),
            p2.declare_blockers([(blocker, attacker)]),
        ])?;
        game.play([
            p2.pass_priority(),
            p1.activate_ability(mountain.ability()).cast(grant).targeting(attacker).pass_priority(),
            p2.pass_priority(),
        ])?;
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_priority(),
            p2.pass_priority(),
        ])?;

        assert_eq!(game.turn().step, turns::Step::SecondCombatDamage);
        assert!(game.player(p2).cards_in(GRAVEYARD).has(blocker));
        assert!(game.player(p1).cards_in(BATTLEFIELD).has(attacker));
        assert_eq!(game.life(p2), 20);

        game.play([p1.pass_priority(), p2.pass_priority()])?;

        assert_eq!(game.turn().step, turns::Step::EndOfCombat);
        assert!(game.player(p1).cards_in(BATTLEFIELD).has(attacker));
        assert_eq!(game.life(p2), 20);
        Ok(())
    }
}
