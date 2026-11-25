use crate::events;
use bevy::ecs::schedule::{IntoScheduleConfigs, ScheduleConfigs, SystemCondition};
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::{Entity, MessageReader, MessageWriter, ParamSet, Query, Res, With};

use crate::Attempt;
use crate::actions;
use crate::cards;
use crate::damage;
use crate::players;
use crate::turns;

type CombatLookups<'w, 's> = (
    Query<
        'w,
        's,
        (
            Entity,
            &'s cards::Blocking,
            Option<&'s damage::Damage>,
            Option<&'s cards::EffectivePT>,
        ),
    >,
    Query<'w, 's, &'s cards::Owner>,
);

/// 510.1. First, the active player announces how each attacking creature assigns its combat damage,
/// then the defending player announces how each blocking creature assigns its combat damage. This
/// turn-based action doesn’t use the stack. A player assigns a creature’s combat damage according
/// to the following rules:
///
/// 510.1a Each attacking creature and each blocking creature assigns combat damage equal to its
/// power. Creatures that would assign 0 or less damage this way don’t assign combat damage at all.
///
/// 510.1b An unblocked creature assigns its combat damage to the player, planeswalker, or battle
/// it’s attacking. If it isn’t currently attacking anything (if, for example, it was attacking a
/// planeswalker that has left the battlefield), it assigns no combat damage.
///
/// 510.1c A blocked creature assigns its combat damage to the creatures blocking it. If no
/// creatures are currently blocking it (if, for example, they were destroyed or removed from
/// combat), it assigns no combat damage. If exactly one creature is blocking it, it assigns all its
/// combat damage to that creature. If two or more creatures are blocking it, it assigns its combat
/// damage to those creatures divided as its controller chooses among them.
///
/// 510.1d A blocking creature assigns combat damage to the creatures it’s blocking. If it isn’t
/// currently blocking any creatures (if, for example, they were destroyed or removed from combat),
/// it assigns no combat damage. If it’s blocking exactly one creature, it assigns all its combat
/// damage to that creature. If it’s blocking two or more creatures, it assigns its combat damage
/// divided as its controller chooses among them.
///
/// 510.2. Second, all combat damage that’s been assigned is dealt simultaneously. This turn-based
/// action doesn’t use the stack. No player has the chance to cast spells or activate abilities
/// between the time combat damage is assigned and the time it’s dealt.
///
/// 702.19b The controller of an attacking creature with trample first assigns damage to the
/// creature(s) blocking it. Once all those blocking creatures are assigned lethal damage, any
/// excess damage is assigned as its controller chooses among those blocking creatures and the
/// player, planeswalker, or battle the creature is attacking. When checking for assigned lethal
/// damage, take into account damage already marked on the creature and damage from other creatures
/// that’s being assigned during the same combat damage step, but not any abilities or effects that
/// might change the amount of damage that’s actually dealt. The attacking creature’s controller
/// need not assign lethal damage to all those blocking creatures but in that case can’t assign any
/// damage to the player or planeswalker it’s attacking.
///
/// 702.19d If an attacking creature with trample or trample over planeswalkers is blocked, but
/// there are no creatures blocking it when damage is assigned, its damage is assigned to the
/// defending player and/or planeswalker as though all blocking creatures have been assigned lethal
/// damage.
///
/// 702.2c Any nonzero amount of combat damage assigned to a creature by a source with deathtouch
/// is considered to be lethal damage for the purposes of determining if a proposed combat damage
/// assignment is valid, regardless of that creature’s toughness. See rules 510.1c–d.
///
/// # Notes
///
/// Scope: unblocked creatures deal damage to the defending player (two-player games; planeswalker
/// and battle targets are deferred), single-blocker and single-blocked cases deal damage both ways,
/// and creatures with power 0 or less assign nothing. An attacker blocked by two or more creatures
/// assigns its damage divided as its controller chooses among them; each of its blockers still
/// assigns damage to it. Trample: a single blocker is assigned lethal damage first (toughness minus
/// damage already marked on it) and any excess goes to the defending player; a blocked attacker
/// with no blockers left assigns all its damage to the defending player.
pub fn r510_1_r510_1a_r510_1b_r510_1c_r510_1d_r510_2_r702_2c_r702_19b_r702_19d_combat_damage()
-> ScheduleConfigs<ScheduleSystem> {
    (
        announce_combat_damage.run_if(
            turns::Step::CombatDamage.begins().or_eager(turns::Step::SecondCombatDamage.begins()),
        ),
        deal_assigned_combat_damage,
    )
        .chain()
}

fn announce_combat_damage(
    mut asks: MessageWriter<actions::Ask>,
    mut dealt: MessageWriter<Attempt<events::DealDamage>>,
    attackers: Query<
        (
            Entity,
            &cards::CardDefinition,
            Option<&cards::Blocked>,
            cards::CurrentMarkers<'_>,
        ),
        With<cards::Attacking>,
    >,
    mut lookups: ParamSet<CombatLookups<'_, '_>>,
    defenders: Query<Entity, With<players::DefendingPlayer>>,
    state: Res<turns::TurnState>,
    participants: Res<turns::CombatDamageParticipants>,
) {
    let state = state.into_inner();
    let participants = participants.into_inner();
    let Some(defending) = defenders.iter().next() else {
        return;
    };
    let mut needed: Vec<actions::RequiredCombatDamage> = Vec::new();
    for (attacker, definition, blocked, markers) in attackers.iter() {
        let Some(effective) = markers.effective else {
            continue;
        };
        let amount = u32::try_from(effective.power).unwrap_or(0);
        if blocked.is_none() {
            if amount == 0 || !participants.contains(&attacker) {
                continue;
            }
            dealt.write(Attempt(events::DealDamage {
                source: attacker,
                recipient: defending,
                amount,
                controller: recipient_controller(&lookups.p1(), defending),
            }));
        } else {
            for write in definition.assigned_blocked_damage(
                attacker,
                amount,
                defending,
                &lookups.p0(),
                participants,
                &mut needed,
                markers.deathtouch,
            ) {
                dealt.write(announce_damage(write, &lookups.p1()));
            }
        }
    }
    if !needed.is_empty() {
        asks.write(actions::Ask(actions::Choice::CombatDamage(
            actions::CombatDamageChoice {
                player: state.turn.active,
                needed,
            },
        )));
    }
}

fn deal_assigned_combat_damage(
    mut answers: MessageReader<actions::Answered>,
    mut dealt: MessageWriter<Attempt<events::DealDamage>>,
    owners: Query<&cards::Owner>,
) {
    for answered in answers.read() {
        let actions::Choice::CombatDamage(choice) = &answered.choice else {
            continue;
        };
        let actions::Action::AssignCombatDamage(assignments) = &answered.answer else {
            continue;
        };
        if let Some(writes) = choice.assigned_damage(assignments) {
            for write in writes {
                dealt.write(announce_damage(write, &owners));
            }
        }
    }
}

fn announce_damage(
    assigned: damage::AssignedDamage,
    owners: &Query<&cards::Owner>,
) -> Attempt<events::DealDamage> {
    Attempt(events::DealDamage {
        source: assigned.source,
        recipient: assigned.target,
        amount: assigned.amount,
        controller: recipient_controller(owners, assigned.target),
    })
}

fn recipient_controller(owners: &Query<&cards::Owner>, recipient: Entity) -> players::PlayerId {
    owners.get(recipient).map_or(players::PlayerId(recipient), |owner| **owner)
}

#[cfg(test)]
mod tests {
    use crate::GRAVEYARD;
    use crate::mana;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn blocked_creatures_deal_damage_to_each_other() -> TestResult {
        use crate::BATTLEFIELD;

        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TYPHOID_RATS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [rats] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TYPHOID_RATS)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([rats]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])?;
        game.play([p1.pass_priority(), p2.declare_blockers([(bears, rats)])])?;
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_priority(),
            p2.pass_priority(),
        ])?;

        assert!(game.player(p1).cards_in(GRAVEYARD).has(rats));
        assert!(game.player(p2).cards_in(GRAVEYARD).has(bears));
        Ok(())
    }

    #[test]
    fn unblocked_creature_deals_damage_to_the_defending_player() -> TestResult {
        use crate::BATTLEFIELD;

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
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_priority(),
            p2.pass_priority(),
        ])?;

        assert_eq!(game.player(p2).life(), 18);
        assert!(game.player(p1).cards_in(BATTLEFIELD).has(bears));
        Ok(())
    }

    #[test]
    fn a_pumped_creature_assigns_combat_damage_equal_to_its_modified_power() -> TestResult {
        use crate::BATTLEFIELD;
        use crate::HAND;

        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS]).hand([GIANT_GROWTH]).mana_pool(mana![G]),
            P2,
        ]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [growth] = game.player(p1).cards_in(HAND).exactly::<1>(GIANT_GROWTH)?;

        game.play([
            p1.cast(growth).targeting(bears).pass_priority(),
            p2.pass_priority(),
        ])?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([bears]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_priority(),
            p2.pass_priority(),
        ])?;

        assert_eq!(game.player(p2).life(), 15);
        Ok(())
    }

    #[test]
    fn a_blocked_creature_divides_its_damage_among_its_blockers() -> TestResult {
        use crate::BATTLEFIELD;
        use crate::damage;

        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS]),
            P2.battlefield([DARKSTEEL_MYR, DARKSTEEL_MYR]),
        ]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [myr_1, myr_2] = game.player(p2).cards_in(BATTLEFIELD).exactly::<2>(DARKSTEEL_MYR)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([bears]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])?;
        game.play([
            p1.pass_priority(),
            p2.declare_blockers([(myr_1, bears), (myr_2, bears)]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
        ])?;

        game.play([p1.assign_combat_damage([(myr_1.into(), 1), (myr_2.into(), 1)])])?;
        game.play([p1.pass_priority(), p2.pass_priority()])?;

        assert_eq!(game.get::<damage::Damage>(myr_1), Some(damage::Damage(1)));
        assert_eq!(game.get::<damage::Damage>(myr_2), Some(damage::Damage(1)));
        Ok(())
    }

    #[test]
    fn trample_assigns_excess_damage_to_the_defending_player() -> TestResult {
        use crate::BATTLEFIELD;
        use crate::GRAVEYARD;

        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GARRUKS_COMPANION]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [companion] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GARRUKS_COMPANION)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([companion]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])?;
        game.play([
            p1.pass_priority(),
            p2.declare_blockers([(bears, companion)]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_priority(),
            p2.pass_priority(),
        ])?;

        assert_eq!(game.life(p2), 19);
        assert!(game.player(p2).cards_in(GRAVEYARD).has(bears));
        assert!(game.player(p1).cards_in(GRAVEYARD).has(companion));
        Ok(())
    }

    #[test]
    fn trample_assigns_excess_damage_past_two_blockers_to_the_defending_player() -> TestResult {
        use crate::BATTLEFIELD;
        use crate::turns;

        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GARRUKS_COMPANION]),
            P2.battlefield([TYPHOID_RATS, TYPHOID_RATS]),
        ]);
        let [companion] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GARRUKS_COMPANION)?;
        let [rats_1, rats_2] = game.player(p2).cards_in(BATTLEFIELD).exactly::<2>(TYPHOID_RATS)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([companion]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])?;
        game.play([
            p1.pass_priority(),
            p2.declare_blockers([(rats_1, companion), (rats_2, companion)]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
        ])?;
        game.play([p1.assign_combat_damage([
            (rats_1.into(), 1),
            (rats_2.into(), 1),
            (p2.into(), 1),
        ])])?;
        game.play([p1.pass_priority()])?;
        game.play([p2.arrange_graveyard([rats_1, rats_2])])?;
        game.play([p2.pass_priority()])?;

        assert_eq!(game.player(p2).life(), 19);
        assert!(game.player(p2).cards_in(GRAVEYARD).has(rats_1));
        assert!(game.player(p2).cards_in(GRAVEYARD).has(rats_2));
        Ok(())
    }

    #[test]
    fn deathtouch_assigns_one_damage_to_a_blocker_and_tramples_the_rest() -> TestResult {
        use crate::BATTLEFIELD;
        use crate::GRAVEYARD;
        use crate::HAND;

        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GARRUKS_COMPANION]).hand([BLADEBRAND]).mana_pool(mana![1, B]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [companion] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GARRUKS_COMPANION)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [bladebrand] = game.player(p1).cards_in(HAND).exactly::<1>(BLADEBRAND)?;
        game.play([
            p1.cast(bladebrand).targeting(companion).pass_priority(),
            p2.pass_priority(),
        ])?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([companion]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])?;
        game.play([
            p1.pass_priority(),
            p2.declare_blockers([(bears, companion)]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_priority(),
            p2.pass_priority(),
        ])?;

        assert_eq!(game.damage(bears), 1);
        assert_eq!(game.life(p2), 18);
        assert!(game.player(p2).cards_in(GRAVEYARD).has(bears));
        Ok(())
    }

    #[test]
    fn creature_with_zero_power_assigns_no_damage() -> TestResult {
        use crate::BATTLEFIELD;

        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([DARKSTEEL_MYR]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [myr] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(DARKSTEEL_MYR)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([myr]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_priority(),
            p2.pass_priority(),
        ])?;

        assert_eq!(game.player(p2).life(), 20);
        assert!(game.player(p1).cards_in(BATTLEFIELD).has(myr));
        Ok(())
    }
}
