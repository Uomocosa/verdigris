use bevy::prelude::{Commands, Entity, Has, MessageReader, MessageWriter, Query, Res};

use crate::BATTLEFIELD;
use crate::actions;
use crate::cards;
use crate::rules;
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
///
/// # See also
///
/// 509.1c The defending player checks each creature they control to see whether it’s affected by any
/// requirements (effects that say a creature must block, or that it must block if some condition is
/// met). If the number of requirements that are being obeyed is fewer than the maximum possible
/// number of requirements that could be obeyed without disobeying any restrictions, the declaration
/// of blockers is illegal. If a creature can’t block unless a player pays a cost, that player is not
/// required to pay that cost, even if blocking with that creature would increase the number of
/// requirements being obeyed. If a requirement that says a creature blocks if able during a certain
/// turn refers to a turn with multiple combat phases, the creature blocks if able during each declare
/// blockers step in that turn.
pub fn r509_1a_r509_1g_r509_1h_r702_9b_r702_17b_declare_blockers(
    mut actions: MessageReader<actions::PlayerAction>,
    mut rejected: MessageWriter<actions::Rejected>,
    mut commands: Commands,
    priority: Res<turns::Priority>,
    turn: Res<turns::Turn>,
    cards: Query<(
        Entity,
        &cards::CardDefinition,
        &cards::Owner,
        &zones::InZone,
        Option<&cards::Tapped>,
    )>,
    attacking: Query<(&cards::Attacking, Has<cards::Flying>)>,
    flight: Query<(Has<cards::Flying>, Has<cards::Reach>)>,
    answers: rules::Rules,
    zones: zones::ZoneLookup,
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
        let Some(battlefield) = zones.find(BATTLEFIELD, Some(action.player)) else {
            rejected.write(actions::Rejected::CannotBlock {
                assignments: assignments.clone(),
            });
            continue;
        };
        let all_legal = assignments.iter().all(|assignment| {
            let blocker_legal = cards.get(assignment.blocker).is_ok_and(
                |(_, definition, owner, in_zone, tapped)| {
                    definition.type_line.types.contains(&cards::CardType::Creature)
                        && **owner == action.player
                        && **in_zone == battlefield
                        && tapped.is_none()
                },
            );
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
        let requirement_unmet = cards.iter().any(|(entity, definition, owner, in_zone, tapped)| {
            **owner == action.player
                && **in_zone == battlefield
                && definition.type_line.types.contains(&cards::CardType::Creature)
                && answers.ask::<rules::MustBlock>(entity)
                && tapped.is_none()
                && !assignments.iter().any(|assignment| assignment.blocker == entity)
                && attacking.iter().any(|(_, attacker_flies)| {
                    !attacker_flies
                        || flight.get(entity).is_ok_and(|(blocker_flies, blocker_reach)| {
                            blocker_flies || blocker_reach
                        })
                })
        });
        if !all_legal || requirement_unmet {
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
    use crate::HAND;
    use crate::actions;
    use crate::cards;
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
            game.get::<crate::cards::Blocking>(bears),
            Some(crate::cards::Blocking(rats.into()))
        );
        assert!(game.has::<crate::cards::Blocked>(rats));
        Ok(())
    }

    #[test]
    fn a_tapped_creature_cannot_be_declared_as_a_blocker() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TYPHOID_RATS]),
            P2.battlefield_tapped([GRIZZLY_BEARS]),
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
        game.play([p1.pass_priority()])?;
        let assignments = vec![actions::BlockAssignment {
            blocker: bears.into(),
            attacker: rats.into(),
        }];

        game.act(p2, actions::Action::DeclareBlockers(assignments.clone()));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotBlock { assignments })
        );
        assert!(!game.is_blocking(bears));
        Ok(())
    }

    #[test]
    fn a_creature_that_is_not_attacking_cannot_be_blocked() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TYPHOID_RATS, GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [rats] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TYPHOID_RATS)?;
        let [bears_1] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [bears_2] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
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
        assert!(!game.is_attacking(bears_1));
        let assignments = vec![actions::BlockAssignment {
            blocker: bears_2.into(),
            attacker: bears_1.into(),
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
        assert!(!game.is_blocking(bears));
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
            game.get::<crate::cards::Blocking>(spider),
            Some(crate::cards::Blocking(drake.into()))
        );
        Ok(())
    }

    #[test]
    fn grizzly_bears_with_jump_cannot_be_blocked_by_a_creature_without_flying_or_reach()
    -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([JUMP]).battlefield([GRIZZLY_BEARS, TUNDRA]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [jump] = game.player(p1).cards_in(HAND).exactly::<1>(JUMP)?;
        let [bears_1] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [tundra] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TUNDRA)?;
        let [bears_2] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([bears_1]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])?;
        game.play([
            p1.activate_ability(tundra.mana_ability(cards::Color::Blue))
                .cast(jump)
                .targeting(bears_1)
                .pass_priority(),
            p2.pass_priority(),
        ])?;
        game.play([p1.pass_priority()])?;
        let assignments = vec![actions::BlockAssignment {
            blocker: bears_2.into(),
            attacker: bears_1.into(),
        }];

        game.act(p2, actions::Action::DeclareBlockers(assignments.clone()));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotBlock { assignments })
        );
        assert!(!game.is_blocking(bears_2));
        Ok(())
    }

    #[test]
    fn a_creature_that_must_block_cannot_be_left_out_of_the_declaration() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS]),
            P2.battlefield([IRON_GOLEM]),
        ]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [golem] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(IRON_GOLEM)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([bears]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])?;
        game.play([p1.pass_priority()])?;

        game.act(p2, actions::Action::DeclareBlockers(vec![]));
        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotBlock {
                assignments: vec![]
            })
        );
        assert!(!game.is_blocking(golem));

        game.play([p2.declare_blockers([(golem, bears)])])?;
        assert!(game.is_blocking(golem));
        Ok(())
    }

    #[test]
    fn a_static_must_block_grant_forces_a_block() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS, TEST_TAUNTING_FIELD]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [bears_1] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [bears_2] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([bears_1]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])?;
        game.play([p1.pass_priority()])?;

        game.act(p2, actions::Action::DeclareBlockers(vec![]));
        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotBlock {
                assignments: vec![]
            })
        );
        assert!(!game.is_blocking(bears_2));

        game.play([p2.declare_blockers([(bears_2, bears_1)])])?;
        assert!(game.is_blocking(bears_2));
        Ok(())
    }

    #[test]
    fn a_static_must_block_rule_answer_forces_a_block() -> TestResult {
        use bevy::ecs::system::RunSystemOnce;

        use crate::rules;

        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS, TEST_LURE_OF_BATTLE]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [bears_1] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [bears_2] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([bears_1]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])?;
        game.play([p1.pass_priority()])?;

        game.act(p2, actions::Action::DeclareBlockers(vec![]));
        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotBlock {
                assignments: vec![]
            })
        );
        assert!(!game.is_blocking(bears_2));

        game.play([p2.declare_blockers([(bears_2, bears_1)])])?;
        assert!(game.is_blocking(bears_2));
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(answers.ask::<rules::MustBlock>(bears_2.into()));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn a_static_flying_loss_allows_a_ground_creature_to_block() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([WIND_DRAKE]),
            P2.battlefield([GRIZZLY_BEARS, TEST_GROUNDING_MIST]),
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
        game.play([p1.pass_priority(), p2.declare_blockers([(bears, drake)])])?;

        assert!(game.is_blocking(bears));
        Ok(())
    }
}
