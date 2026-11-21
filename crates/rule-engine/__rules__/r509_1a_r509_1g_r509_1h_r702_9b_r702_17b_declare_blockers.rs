use bevy::prelude::{Commands, Entity, Has, MessageReader, MessageWriter, Query, Res};

use crate::BATTLEFIELD;
use crate::actions;
use crate::cards;
use crate::turns;
use crate::zones;

/// 509.1a The defending player chooses which creatures they control, if any, will block. The chosen
/// creatures must be untapped and they can’t also be battles. For each of the chosen creatures, the
/// defending player chooses one creature for it to block that’s attacking that player, a
/// planeswalker they control, or a battle they protect.
///
/// 509.1g Each chosen creature still controlled by the defending player becomes a blocking
/// creature. Each one is blocking the attacking creatures chosen for it. It remains a blocking
/// creature until it’s removed from combat or the combat phase ends, whichever comes first. See
/// rule 506.4.
///
/// 509.1h An attacking creature with one or more creatures declared as blockers for it becomes a
/// blocked creature; one with no creatures declared as blockers for it becomes an unblocked
/// creature. This remains unchanged until the creature is removed from combat, an effect says that
/// it becomes blocked or unblocked, or the combat phase ends, whichever comes first. A creature
/// remains blocked even if all the creatures blocking it are removed from combat.
///
/// 702.9b A creature with flying can’t be blocked except by creatures with flying and/or reach. A
/// creature with flying can block a creature with or without flying. (See rule 509, “Declare
/// Blockers Step,” and rule 702.17, “Reach.”)
///
/// 702.17b A creature with flying can’t be blocked except by creatures with flying and/or reach.
/// (See rule 509, “Declare Blockers Step,” and rule 702.9, “Flying.”)
pub fn r509_1a_r509_1g_r509_1h_r702_9b_r702_17b_declare_blockers(
    mut actions: MessageReader<actions::PlayerAction>,
    mut rejected: MessageWriter<actions::Rejected>,
    mut commands: Commands,
    priority: Res<turns::Priority>,
    turn: Res<turns::Turn>,
    cards: Query<(
        &cards::CardDefinition,
        &cards::Owner,
        &zones::InZone,
        Option<&cards::Tapped>,
    )>,
    attacking: Query<(&cards::Attacking, Has<cards::Flying>)>,
    flight: Query<(Has<cards::Flying>, Has<cards::Reach>)>,
    zone_entities: Query<(Entity, &zones::Zone, Option<&cards::Owner>)>,
) {
    let priority = priority.into_inner();
    let turn = turn.into_inner();
    for action in actions.read() {
        let actions::Action::DeclareBlockers(assignments) = &action.action else {
            continue;
        };
        if turn.step != turns::Step::DeclareBlockers
            || action.player != priority.player
            || action.player == turn.active
        {
            rejected.write(actions::Rejected::CannotBlock {
                assignments: assignments.clone(),
            });
            continue;
        }
        let Some(battlefield) = BATTLEFIELD.find_entity(Some(action.player), &zone_entities) else {
            rejected.write(actions::Rejected::CannotBlock {
                assignments: assignments.clone(),
            });
            continue;
        };
        let all_legal = assignments.iter().all(|assignment| {
            let blocker_legal =
                cards.get(assignment.blocker).is_ok_and(|(definition, owner, in_zone, tapped)| {
                    definition.types.contains(&cards::CardType::Creature)
                        && **owner == action.player
                        && **in_zone == battlefield
                        && tapped.is_none()
                });
            let attacker_legal =
                attacking.get(assignment.attacker).is_ok_and(|(_, attacker_flies)| {
                    if !attacker_flies {
                        return true;
                    }
                    flight
                        .get(assignment.blocker)
                        .is_ok_and(|(blocker_flies, blocker_reach)| blocker_flies || blocker_reach)
                });
            blocker_legal && attacker_legal
        });
        if !all_legal {
            rejected.write(actions::Rejected::CannotBlock {
                assignments: assignments.clone(),
            });
            continue;
        }
        for assignment in assignments {
            commands.entity(assignment.blocker).insert(cards::Blocking(assignment.attacker));
            commands.entity(assignment.attacker).insert(cards::Blocked);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::actions;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn declared_blockers_become_blocking_and_attacker_blocked() -> TestResult {
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

        assert_eq!(
            game.app.world().get::<crate::cards::Blocking>(bears.into()),
            Some(&crate::cards::Blocking(rats.into()))
        );
        assert!(game.app.world().get::<crate::cards::Blocked>(rats.into()).is_some());
        Ok(())
    }

    #[test]
    fn a_tapped_creature_cannot_be_declared_as_a_blocker() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TYPHOID_RATS]),
            P2.battlefield_tapped([GRIZZLY_BEARS]),
        ]);
        let [rats] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TYPHOID_RATS)?;
        let [tapped] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([rats]),
        ])?;

        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])?;
        game.play([p1.pass_priority()])?;
        let assignments = vec![actions::BlockAssignment {
            blocker: tapped.into(),
            attacker: rats.into(),
        }];

        game.act(p2, actions::Action::DeclareBlockers(assignments.clone()));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotBlock { assignments })
        );
        assert!(game.app.world().get::<crate::cards::Blocking>(tapped.into()).is_none());
        Ok(())
    }

    #[test]
    fn a_creature_that_is_not_attacking_cannot_be_blocked() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TYPHOID_RATS, GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [rats] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TYPHOID_RATS)?;
        let [idle] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
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
        game.play([p1.pass_priority()])?;
        assert!(game.app.world().get::<crate::cards::Attacking>(idle.into()).is_none());
        let assignments = vec![actions::BlockAssignment {
            blocker: bears.into(),
            attacker: idle.into(),
        }];

        game.act(p2, actions::Action::DeclareBlockers(assignments.clone()));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotBlock { assignments })
        );
        Ok(())
    }

    #[test]
    fn a_creature_without_flying_or_reach_cannot_block_a_flyer() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([WIND_DRAKE]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [drake] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(WIND_DRAKE)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([drake]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])?;
        game.play([p1.pass_priority()])?;
        let assignments = vec![actions::BlockAssignment {
            blocker: bears.into(),
            attacker: drake.into(),
        }];

        game.act(p2, actions::Action::DeclareBlockers(assignments.clone()));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotBlock { assignments })
        );
        assert!(game.app.world().get::<crate::cards::Blocking>(bears.into()).is_none());
        Ok(())
    }

    #[test]
    fn a_creature_with_reach_can_block_a_flyer() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.battlefield([WIND_DRAKE]), P2.battlefield([GIANT_SPIDER])]);
        let [drake] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(WIND_DRAKE)?;
        let [spider] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GIANT_SPIDER)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([drake]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])?;
        game.play([p1.pass_priority(), p2.declare_blockers([(spider, drake)])])?;

        assert_eq!(
            game.app.world().get::<crate::cards::Blocking>(spider.into()),
            Some(&crate::cards::Blocking(drake.into()))
        );
        Ok(())
    }
}
