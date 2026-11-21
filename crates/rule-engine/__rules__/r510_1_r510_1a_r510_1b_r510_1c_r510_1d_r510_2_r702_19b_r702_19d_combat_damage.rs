use bevy::prelude::{Entity, MessageReader, MessageWriter, Query, Res, ResMut, With};

use crate::actions;
use crate::cards;
use crate::damage;
use crate::players;
use crate::turns;

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
/// # Notes
///
/// Scope: unblocked creatures deal damage to the defending player (two-player games; planeswalker
/// and battle targets are deferred), single-blocker and single-blocked cases deal damage both ways,
/// and creatures with power 0 or less assign nothing. An attacker blocked by two or more creatures
/// assigns its damage divided as its controller chooses among them; each of its blockers still
/// assigns damage to it. Trample: a single blocker is assigned lethal damage first (toughness minus
/// damage already marked on it) and any excess goes to the defending player; a blocked attacker
/// with no blockers left assigns all its damage to the defending player.
pub fn r510_1_r510_1a_r510_1b_r510_1c_r510_1d_r510_2_r702_19b_r702_19d_combat_damage(
    mut step_started: MessageReader<turns::StepStarted>,
    mut actions: MessageReader<actions::PlayerAction>,
    mut rejected: MessageWriter<actions::Rejected>,
    mut pending: ResMut<actions::PendingChoice>,
    mut dealt: MessageWriter<damage::DamageDealt>,
    attackers: Query<
        (
            Entity,
            &cards::CardDefinition,
            &cards::EffectivePT,
            Option<&cards::Blocked>,
        ),
        With<cards::Attacking>,
    >,
    blockers: Query<(
        Entity,
        &cards::Blocking,
        Option<&damage::Damage>,
        Option<&cards::EffectivePT>,
    )>,
    defenders: Query<Entity, With<players::DefendingPlayer>>,
    turn: Res<turns::Turn>,
) {
    let turn = turn.into_inner();
    for step in step_started.read() {
        if **step != turns::Step::CombatDamage {
            continue;
        }
        let Some(defending) = defenders.iter().next() else {
            continue;
        };
        let mut needed: Vec<actions::RequiredCombatDamage> = Vec::new();
        for (attacker, definition, effective, blocked) in attackers.iter() {
            let power_toughness = **effective;
            if power_toughness.power <= 0 {
                continue;
            }
            let Ok(amount) = u32::try_from(power_toughness.power) else {
                continue;
            };
            if blocked.is_none() {
                dealt.write(damage::DamageDealt {
                    source: attacker,
                    target: defending,
                    amount,
                });
            } else {
                for write in definition.assigned_blocked_damage(
                    attacker,
                    amount,
                    defending,
                    &blockers,
                    &mut needed,
                ) {
                    dealt.write(write);
                }
            }
        }
        if !needed.is_empty() {
            *pending = actions::PendingChoice(Some(actions::Choice::CombatDamage(
                actions::CombatDamageChoice {
                    player: turn.active,
                    needed,
                },
            )));
        }
    }
    for action in actions.read() {
        let actions::Action::AssignCombatDamage(assignments) = &action.action else {
            continue;
        };
        let Some(actions::Choice::CombatDamage(choice)) = (*pending).as_ref().cloned() else {
            rejected.write(actions::Rejected::InvalidCombatDamage {
                player: action.player,
            });
            continue;
        };
        let Some(writes) = choice.assigned_damage(action.player, assignments) else {
            rejected.write(actions::Rejected::InvalidCombatDamage {
                player: action.player,
            });
            continue;
        };
        for write in writes {
            dealt.write(write);
        }
        *pending = actions::PendingChoice(None);
    }
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
        use crate::actions;
        use crate::damage;

        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS]),
            P2.battlefield([DARKSTEEL_MYR, DARKSTEEL_MYR]),
        ]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [first, second] = game.player(p2).cards_in(BATTLEFIELD).exactly::<2>(DARKSTEEL_MYR)?;
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
            p2.declare_blockers([(first, bears), (second, bears)]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
        ])?;

        game.act(
            p1,
            actions::Action::AssignCombatDamage(vec![
                actions::DamageAssignment {
                    target: first.into(),
                    amount: 1,
                },
                actions::DamageAssignment {
                    target: second.into(),
                    amount: 1,
                },
            ]),
        );
        game.play([p1.pass_priority(), p2.pass_priority()])?;

        assert_eq!(game.get::<damage::Damage>(first), Some(damage::Damage(1)));
        assert_eq!(game.get::<damage::Damage>(second), Some(damage::Damage(1)));
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
        use crate::actions;
        use crate::turns;

        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GARRUKS_COMPANION]),
            P2.battlefield([TYPHOID_RATS, TYPHOID_RATS]),
        ]);
        let [companion] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GARRUKS_COMPANION)?;
        let [first, second] = game.player(p2).cards_in(BATTLEFIELD).exactly::<2>(TYPHOID_RATS)?;
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
            p2.declare_blockers([(first, companion), (second, companion)]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
        ])?;
        game.act(
            p1,
            actions::Action::AssignCombatDamage(vec![
                actions::DamageAssignment {
                    target: first.into(),
                    amount: 1,
                },
                actions::DamageAssignment {
                    target: second.into(),
                    amount: 1,
                },
                actions::DamageAssignment {
                    target: p2.into(),
                    amount: 1,
                },
            ]),
        );
        game.play([p1.pass_priority(), p2.pass_priority()])?;

        assert_eq!(game.player(p2).life(), 19);
        assert!(game.player(p2).cards_in(GRAVEYARD).has(first));
        assert!(game.player(p2).cards_in(GRAVEYARD).has(second));
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
