use bevy::prelude::{Entity, MessageReader, MessageWriter, Query, With};

use crate::Attempt;
use crate::Game;
use crate::actions;
use crate::cards;
use crate::events;
use crate::rules;
use crate::turns;

/// 509.1a The defending player chooses which creatures they control, if any, will block. The chosen
/// creatures must be untapped and they can’t also be battles. For each of the chosen creatures, the
/// defending player chooses one creature for it to block that’s attacking that player, a
/// planeswalker they control, or a battle they protect.
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
///
/// 702.9b A creature with flying can’t be blocked except by creatures with flying and/or reach. A
/// creature with flying can block a creature with or without flying. (See rule 509, “Declare
/// Blockers Step,” and rule 702.17, “Reach.”)
///
/// 702.17b A creature with flying can’t be blocked except by creatures with flying and/or reach.
/// (See rule 509, “Declare Blockers Step,” and rule 702.9, “Flying.”)
pub fn r509_1a_declare_blockers(
    mut actions: MessageReader<actions::PlayerAction>,
    mut rejected: MessageWriter<actions::Rejected>,
    mut declared: MessageWriter<Attempt<events::DeclareBlocker>>,
    game: Game,
    blockers: Query<(Entity, &cards::Controller), With<cards::Creature>>,
    attacking: Query<Entity, With<cards::Attacking>>,
    answers: rules::Rules,
) {
    for action in actions.read() {
        let actions::Action::DeclareBlockers(assignments) = &action.action else {
            continue;
        };
        if game.turn().step() != Some(turns::Step::DeclareBlockers)
            || game.priority_holder().is_none_or(|holder| holder.id != action.player)
            || game.active_player().is_some_and(|active| active.id == action.player)
        {
            rejected.write(actions::Rejected::CannotBlock {
                assignments: assignments.clone(),
            });
            continue;
        }
        let all_legal = assignments.iter().all(|assignment| {
            attacking.get(assignment.attacker).is_ok()
                && answers.legal::<events::DeclareBlocker>(&events::DeclareBlocker {
                    blocker: assignment.blocker,
                    attacker: assignment.attacker,
                    player: action.player,
                })
        });
        let blocks_one_creature = assignments.iter().all(|assignment| {
            assignments.iter().filter(|other| other.blocker == assignment.blocker).count() == 1
        });
        let requirement_unmet = blockers.iter().any(|(entity, controller)| {
            **controller == action.player
                && answers.required::<events::DeclareBlocker>(entity)
                && !assignments.iter().any(|assignment| assignment.blocker == entity)
                && attacking.iter().any(|attacker| {
                    answers.legal::<events::DeclareBlocker>(&events::DeclareBlocker {
                        blocker: entity,
                        attacker,
                        player: action.player,
                    })
                })
        });
        if !all_legal || !blocks_one_creature || requirement_unmet {
            rejected.write(actions::Rejected::CannotBlock {
                assignments: assignments.clone(),
            });
            continue;
        }
        for assignment in assignments {
            declared.write(Attempt(events::DeclareBlocker {
                blocker: assignment.blocker,
                attacker: assignment.attacker,
                player: action.player,
            }));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::HAND;
    use crate::actions;
    use crate::cards;
    use crate::events;
    use crate::testing::*;
    use crate::turns;

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
    fn a_creature_that_waives_untapped_can_block_while_tapped() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TYPHOID_RATS]),
            P2.battlefield_tapped([TEST_TAPPED_SENTRY]),
        ]);
        let [rats] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TYPHOID_RATS)?;
        let [sentry] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(TEST_TAPPED_SENTRY)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([rats]),
        ])?;

        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])?;
        game.play([p1.pass_priority(), p2.declare_blockers([(sentry, rats)])])?;

        assert!(game.is_blocking(sentry));
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
            p1.activate_ability(tundra.mana_ability(cards::ManaType::from(cards::Color::Blue)))
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
                assert!(answers.required::<events::DeclareBlocker>(bears_2.into()));
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

    #[test]
    fn creatures_with_power_two_or_less_cannot_block_under_timid_field() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TYPHOID_RATS]),
            P2.battlefield([GRIZZLY_BEARS, WATCHWOLF, TEST_TIMID_FIELD]),
        ]);
        let [rats] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TYPHOID_RATS)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [wolf] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(WATCHWOLF)?;
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

        game.play([p2.declare_blockers([(wolf, rats)])])?;
        assert!(game.is_blocking(wolf));
        Ok(())
    }

    #[test]
    fn flying_blockers_cannot_block_ground_attackers_under_sky_court() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TYPHOID_RATS]),
            P2.battlefield([WIND_DRAKE, GRIZZLY_BEARS, TEST_SKY_COURT]),
        ]);
        let [rats] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TYPHOID_RATS)?;
        let [drake] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(WIND_DRAKE)?;
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
            blocker: drake.into(),
            attacker: rats.into(),
        }];

        game.act(p2, actions::Action::DeclareBlockers(assignments.clone()));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotBlock { assignments })
        );
        assert!(!game.is_blocking(drake));

        game.play([p2.declare_blockers([(bears, rats)])])?;
        assert!(game.is_blocking(bears));
        Ok(())
    }

    #[test]
    fn an_attacker_that_waives_flying_can_be_blocked_by_a_non_flier() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TEST_LOW_SKIMMER]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [skimmer] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TEST_LOW_SKIMMER)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([skimmer]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
            p1.pass_priority(),
            p2.declare_blockers([(bears, skimmer)]),
        ])?;

        assert!(game.is_blocking(bears));
        Ok(())
    }

    #[test]
    fn a_creature_controlled_by_the_attacker_cannot_block() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TYPHOID_RATS, GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [rats] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TYPHOID_RATS)?;
        let [bears_1] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
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
            blocker: bears_1.into(),
            attacker: rats.into(),
        }];

        game.act(p2, actions::Action::DeclareBlockers(assignments.clone()));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotBlock { assignments })
        );
        assert!(!game.is_blocking(bears_1));
        Ok(())
    }
}
