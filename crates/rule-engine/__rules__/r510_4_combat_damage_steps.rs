use std::collections::BTreeSet;

use bevy::ecs::schedule::{IntoScheduleConfigs, ScheduleConfigs};
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::{Commands, Entity, Has, ParamSet, Query, ResMut};

use crate::GameMut;
use crate::cards;
use crate::events;
use crate::rules;
use crate::turns;
use crate::{ASSIGNS_AGAIN, FIRST_COMBAT_DAMAGE_STEP};

/// 510.4. If at least one attacking or blocking creature has first strike (see rule 702.7) or
/// double strike (see rule 702.4) as the combat damage step begins, the only creatures that assign
/// combat damage in that step are those with first strike or double strike. After that step,
/// instead of proceeding to the end of combat step, the phase gets a second combat damage step.
/// The only creatures that assign combat damage in that step are the remaining attackers and
/// blockers that had neither first strike nor double strike as the first combat damage step began,
/// as well as the remaining attackers and blockers that currently have double strike. After that
/// step, the phase proceeds to the end of combat step.
///
/// # See also
///
/// 506.1. The combat phase has five steps, which proceed in order: beginning of combat, declare
/// attackers, declare blockers, combat damage, and end of combat. The declare blockers and combat
/// damage steps are skipped if no creatures are declared as attackers or put onto the battlefield
/// attacking (see rule 508.8). There are two combat damage steps if any attacking or blocking
/// creature has first strike (see rule 702.7) or double strike (see rule 702.4).
pub fn r510_4_combat_damage_steps() -> ScheduleConfigs<ScheduleSystem> {
    (
        the_first_combat_damage_step.run_if(turns::Step::CombatDamage.begins()),
        the_second_combat_damage_step.run_if(turns::Step::SecondCombatDamage.begins()),
    )
        .chain()
}

fn the_first_combat_damage_step(
    mut commands: Commands,
    mut game_and_answers: ParamSet<(GameMut, rules::Rules)>,
    mut participants: ResMut<turns::CombatDamageParticipants>,
    combatants: Query<Entity, cards::InCombat>,
) {
    let first_step = FIRST_COMBAT_DAMAGE_STEP.definition(!rules::Condition::True);
    let with_first_or_double_strike: Vec<Entity> = {
        let answers = game_and_answers.p1();
        combatants
            .iter()
            .filter(|entity| {
                answers.defines(
                    &first_step,
                    &events::JoinCombatDamageStep { creature: *entity },
                )
            })
            .collect()
    };
    let does_a_second_step_happen = !with_first_or_double_strike.is_empty();
    let participants_this_step: BTreeSet<Entity> = if does_a_second_step_happen {
        with_first_or_double_strike.iter().copied().collect()
    } else {
        combatants.iter().collect()
    };
    if does_a_second_step_happen {
        game_and_answers.p0().add_step_after_this_one(turns::Step::SecondCombatDamage);
        for entity in &with_first_or_double_strike {
            commands.entity(*entity).insert(cards::HadFirstStrikeAsDamageBegan);
        }
    }
    **participants = participants_this_step;
}

fn the_second_combat_damage_step(
    mut participants: ResMut<turns::CombatDamageParticipants>,
    combatants: Query<(Entity, Has<cards::HadFirstStrikeAsDamageBegan>), cards::InCombat>,
    answers: rules::Rules,
) {
    let assigns_again = ASSIGNS_AGAIN.definition(!rules::Condition::True);
    let participants_this_step: BTreeSet<Entity> = combatants
        .iter()
        .filter(|(entity, had_first_strike)| {
            !had_first_strike
                || answers.defines(
                    &assigns_again,
                    &events::JoinCombatDamageStep { creature: *entity },
                )
        })
        .map(|(entity, _)| entity)
        .collect();
    **participants = participants_this_step;
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::cards;
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

    #[test]
    fn a_creature_given_first_strike_after_the_first_combat_damage_step_deals_damage_in_the_second()
    -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([FENCING_ACE, GRIZZLY_BEARS, MOUNTAIN]).hand([TEST_FIRST_STRIKE_GRANT]),
            P2,
        ]);
        let [ace] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(FENCING_ACE)?;
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [mountain] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(MOUNTAIN)?;
        let [grant] = game.player(p1).cards_in(HAND).exactly::<1>(TEST_FIRST_STRIKE_GRANT)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([ace, bears]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
            p1.activate_ability(mountain.ability()).cast(grant).targeting(bears),
        ])?;

        assert_eq!(game.turn().step, turns::Step::CombatDamage);
        assert_eq!(game.life(p2), 19);

        game.play([
            p1.pass_priority(),
            p2.pass_priority(),
            p1.pass_priority(),
            p2.pass_priority(),
        ])?;

        assert_eq!(game.turn().step, turns::Step::SecondCombatDamage);
        assert_eq!(game.life(p2), 19);

        game.play([p1.pass_priority(), p2.pass_priority()])?;

        assert_eq!(game.turn().step, turns::Step::EndOfCombat);
        assert_eq!(game.life(p2), 16);
        Ok(())
    }

    #[test]
    fn a_creature_that_loses_first_strike_after_the_first_combat_damage_step_deals_no_damage_in_the_second()
    -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TEST_FIRST_STRIKE_LORD, GRIZZLY_BEARS]),
            P2.battlefield([TUNDRA]).hand([DEMYSTIFY]),
        ]);
        let [lord] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TEST_FIRST_STRIKE_LORD)?;
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [tundra] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(TUNDRA)?;
        let [demystify] = game.player(p2).cards_in(HAND).exactly::<1>(DEMYSTIFY)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([bears]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_priority(),
            p2.activate_ability(tundra.mana_ability(cards::ManaType::from(cards::Color::White)))
                .cast(demystify)
                .targeting(lord),
        ])?;

        assert_eq!(game.life(p2), 18);

        game.play([
            p2.pass_priority(),
            p1.pass_priority(),
            p1.pass_priority(),
            p2.pass_priority(),
        ])?;

        assert_eq!(game.turn().step, turns::Step::SecondCombatDamage);
        assert!(game.player(p1).cards_in(GRAVEYARD).has(lord));
        assert_eq!(game.life(p2), 18);

        game.play([p1.pass_priority(), p2.pass_priority()])?;

        assert_eq!(game.turn().step, turns::Step::EndOfCombat);
        assert_eq!(game.life(p2), 18);
        Ok(())
    }

    #[test]
    fn a_creature_without_first_or_double_strike_deals_damage_only_in_the_second_step() -> TestResult
    {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.battlefield([FENCING_ACE, GRIZZLY_BEARS]), P2]);
        let [ace] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(FENCING_ACE)?;
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([ace, bears]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_priority(),
            p2.pass_priority(),
        ])?;

        assert_eq!(game.turn().step, turns::Step::SecondCombatDamage);
        assert_eq!(game.life(p2), 19);

        game.play([p1.pass_priority(), p2.pass_priority()])?;

        assert_eq!(game.turn().step, turns::Step::EndOfCombat);
        assert_eq!(game.life(p2), 16);
        Ok(())
    }
}
