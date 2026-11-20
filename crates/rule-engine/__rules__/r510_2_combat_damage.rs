use bevy::prelude::{Entity, MessageReader, MessageWriter, Query, Res, With};

use crate::cards;
use crate::damage;
use crate::players;
use crate::turns;

/// 510.1. First, the active player announces how each attacking creature assigns its combat
/// damage, then the defending player announces how each blocking creature assigns its combat
/// damage. This turn-based action doesn’t use the stack. A player assigns a creature’s combat
/// damage according to the following rules:
///
/// 510.1a Each attacking creature and each blocking creature assigns combat damage equal to its
/// power. Creatures that would assign 0 or less damage this way don’t assign combat damage at
/// all.
///
/// 510.1b An unblocked creature assigns its combat damage to the player, planeswalker, or battle
/// it’s attacking. If it isn’t currently attacking anything (if, for example, it was attacking a
/// planeswalker that has left the battlefield), it assigns no combat damage.
///
/// 510.1c A blocked creature assigns its combat damage to the creatures blocking it. If no
/// creatures are currently blocking it (if, for example, they were destroyed or removed from
/// combat), it assigns no combat damage. If exactly one creature is blocking it, it assigns all
/// its combat damage to that creature. If two or more creatures are blocking it, it assigns its
/// combat damage to those creatures divided as its controller chooses among them.
///
/// 510.1d A blocking creature assigns combat damage to the creatures it’s blocking. If it isn’t
/// currently blocking any creatures (if, for example, they were destroyed or removed from
/// combat), it assigns no combat damage. If it’s blocking exactly one creature, it assigns all
/// its combat damage to that creature. If it’s blocking two or more creatures, it assigns its
/// combat damage divided as its controller chooses among them.
///
/// 510.2. Second, all combat damage that’s been assigned is dealt simultaneously. This
/// turn-based action doesn’t use the stack. No player has the chance to cast spells or activate
/// abilities between the time combat damage is assigned and the time it’s dealt.
///
/// Scope: unblocked creatures deal damage to the defending player (two-player games;
/// planeswalker and battle targets are deferred), single-blocker and single-blocked cases deal
/// damage both ways, and creatures with power 0 or less assign nothing. Division among two or
/// more blockers or blocked creatures (controller choice) is deferred: a blocked creature with
/// two or more blockers assigns no combat damage yet, while each of its blockers still assigns
/// damage to it.
pub fn r510_2_combat_damage(
    mut step_started: MessageReader<turns::StepStarted>,
    mut dealt: MessageWriter<damage::DamageDealt>,
    attackers: Query<
        (Entity, &cards::CardDefinition, Option<&cards::Blocked>),
        With<cards::Attacking>,
    >,
    blockers: Query<(Entity, &cards::CardDefinition, &cards::Blocking)>,
    defenders: Query<Entity, With<players::Player>>,
    turn: Res<turns::Turn>,
) {
    let turn = turn.into_inner();
    for step in step_started.read() {
        if **step != turns::Step::CombatDamage {
            continue;
        }
        let Some(defending) = defenders.iter().find(|player| *player != turn.active) else {
            continue;
        };
        for (attacker, definition, blocked) in attackers.iter() {
            let Some(power_toughness) = definition.power_toughness else {
                continue;
            };
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
                let mut blockers_of: Vec<Entity> = Vec::new();
                for (blocker, blocker_definition, blocking) in blockers.iter() {
                    if blocking.0 != attacker {
                        continue;
                    }
                    blockers_of.push(blocker);
                    let Some(blocker_power_toughness) = blocker_definition.power_toughness else {
                        continue;
                    };
                    if blocker_power_toughness.power <= 0 {
                        continue;
                    }
                    let Ok(blocker_amount) = u32::try_from(blocker_power_toughness.power) else {
                        continue;
                    };
                    dealt.write(damage::DamageDealt {
                        source: blocker,
                        target: attacker,
                        amount: blocker_amount,
                    });
                }
                if let [blocker] = blockers_of.as_slice() {
                    dealt.write(damage::DamageDealt {
                        source: attacker,
                        target: *blocker,
                        amount,
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::GRAVEYARD;
    use crate::actions;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn r510_2_blocked_creatures_deal_damage_to_each_other() {
        use crate::BATTLEFIELD;

        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TYPHOID_RATS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])
        .unwrap();
        let [rats] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TYPHOID_RATS).unwrap();
        game.act(p1, actions::Action::DeclareAttackers(vec![rats.into()]));
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])
        .unwrap();
        game.play([p1.pass_priority()]).unwrap();
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS).unwrap();
        game.act(
            p2,
            actions::Action::DeclareBlockers(vec![actions::BlockAssignment {
                blocker: bears.into(),
                attacker: rats.into(),
            }]),
        );
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_priority(),
            p2.pass_priority(),
        ])
        .unwrap();

        assert_eq!(game.zone_of(rats), GRAVEYARD);
        assert_eq!(game.zone_of(bears), GRAVEYARD);
    }

    #[test]
    fn r510_2_unblocked_creature_deals_damage_to_the_defending_player() {
        use crate::BATTLEFIELD;

        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])
        .unwrap();
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS).unwrap();
        game.act(p1, actions::Action::DeclareAttackers(vec![bears.into()]));
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_priority(),
            p2.pass_priority(),
        ])
        .unwrap();

        assert_eq!(game.player(p2).life(), 18);
        assert_eq!(game.zone_of(bears), BATTLEFIELD);
    }

    #[test]
    fn test_usage() {
        use crate::BATTLEFIELD;

        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])
        .unwrap();
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS).unwrap();
        game.act(p1, actions::Action::DeclareAttackers(vec![bears.into()]));
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_priority(),
            p2.pass_priority(),
        ])
        .unwrap();

        assert_eq!(game.player(p2).life(), 18);
    }
}
