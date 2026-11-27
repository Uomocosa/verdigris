use bevy::prelude::{MessageReader, MessageWriter};

use crate::Attempt;
use crate::Game;
use crate::actions;
use crate::events;
use crate::rules;

/// 508.1a The active player chooses which creatures that they control, if any, will attack. The
/// chosen creatures must be untapped, they can’t also be battles, and each one must either have
/// haste or have been controlled by the active player continuously since the turn began.
///
/// # See also
///
/// 508.1d The active player checks each creature they control to see whether it’s affected by any
/// requirements (effects that say a creature attacks if able, or that it attacks if some condition
/// is met). If the number of requirements that are being obeyed is fewer than the maximum possible
/// number of requirements that could be obeyed without disobeying any restrictions, the declaration
/// of attackers is illegal. If a creature can’t attack unless a player pays a cost, that player is
/// not required to pay that cost, even if attacking with that creature would increase the number of
/// requirements being obeyed. If a requirement that says a creature attacks if able during a
/// certain turn refers to a turn with multiple combat phases, the creature attacks if able during
/// each declare attackers step in that turn.
///
/// 702.10b If a creature has haste, it can attack even if it hasn’t been controlled by its
/// controller continuously since their most recent turn began. (See rule 302.6.)
///
/// 508.1c The active player checks each creature they control to see whether it’s affected by
/// any restrictions (effects that say a creature can’t attack, or that it can’t attack unless
/// some condition is met). If any restrictions are being disobeyed, the declaration of
/// attackers is illegal.
///
/// 302.6. A creature’s activated ability with the tap symbol or the untap symbol in its activation
/// cost can’t be activated unless the creature has been under its controller’s control continuously
/// since their most recent turn began. A creature can’t attack unless it has been under its
/// controller’s control continuously since their most recent turn began. This rule is informally
/// called the “summoning sickness” rule.
pub fn r508_1a_declare_attackers(
    mut actions: MessageReader<actions::PlayerAction>,
    mut rejected: MessageWriter<actions::Rejected>,
    mut declared: MessageWriter<Attempt<events::Attack>>,
    game: Game,
    answers: rules::Rules,
) {
    for action in actions.read() {
        let actions::Action::DeclareAttackers(attackers) = &action.action else {
            continue;
        };
        if let Err(why) = answers.validate_attackers(&game, action.player, attackers) {
            rejected.write(actions::Rejected::CannotAttack {
                attackers: attackers.clone(),
                why,
            });
            continue;
        }
        for attacker in attackers {
            declared.write(Attempt(events::Attack {
                attacker: *attacker,
            }));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::actions;
    use crate::events;
    use crate::keywords;
    use crate::rules;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn a_creature_controlled_by_the_opponent_cannot_attack() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])?;

        game.act(p1, actions::Action::DeclareAttackers(vec![bears.into()]));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotAttack {
                attackers: vec![bears.into()],
                why: vec![actions::DeclarationWhy {
                    creature: Some(bears.into()),
                    why: rules::Why::NotYours,
                }],
            })
        );
        assert!(!game.is_attacking(bears));
        Ok(())
    }

    #[test]
    fn a_noncreature_permanent_cannot_attack() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.battlefield([TUNDRA]), P2]);
        let [tundra] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TUNDRA)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])?;

        game.act(p1, actions::Action::DeclareAttackers(vec![tundra.into()]));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotAttack {
                attackers: vec![tundra.into()],
                why: vec![
                    actions::DeclarationWhy {
                        creature: Some(tundra.into()),
                        why: rules::Why::NotYours,
                    },
                    actions::DeclarationWhy {
                        creature: Some(tundra.into()),
                        why: rules::Why::Restriction(events::attack::CREATURE),
                    },
                ],
            })
        );
        assert!(!game.is_attacking(tundra));
        Ok(())
    }

    #[test]
    fn a_tapped_creature_cannot_be_declared_as_an_attacker() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield_tapped([GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])?;
        game.assert_legal_actions_agree()?;

        game.act(p1, actions::Action::DeclareAttackers(vec![bears.into()]));

        game.app.update();
        let legal = game.app.world().resource::<actions::LegalActions>().clone();
        assert!(legal.attackers.contains(&(
            bears.into(),
            rules::Legality::Illegal(vec![rules::Why::Restriction(events::attack::UNTAPPED)])
        )));
        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotAttack {
                attackers: vec![bears.into()],
                why: vec![actions::DeclarationWhy {
                    creature: Some(bears.into()),
                    why: rules::Why::Restriction(events::attack::UNTAPPED),
                }],
            })
        );
        assert!(!game.is_attacking(bears));
        Ok(())
    }

    #[test]
    fn a_creature_that_entered_this_turn_cannot_attack() -> TestResult {
        use crate::HAND;
        use crate::mana;

        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([WALKING_CORPSE]).mana_pool(mana![1, B]), P2]);
        let [corpse] = game.player(p1).cards_in(HAND).exactly::<1>(WALKING_CORPSE)?;
        game.play([p1.cast(corpse).pass_priority(), p2.pass_priority()])?;
        let [corpse_on_battlefield] =
            game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(WALKING_CORPSE)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])?;

        game.act(
            p1,
            actions::Action::DeclareAttackers(vec![corpse_on_battlefield.into()]),
        );

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotAttack {
                attackers: vec![corpse_on_battlefield.into()],
                why: vec![actions::DeclarationWhy {
                    creature: Some(corpse_on_battlefield.into()),
                    why: rules::Why::Restriction(events::attack::SUMMONING_SICK),
                }],
            })
        );
        assert!(!game.is_attacking(corpse_on_battlefield));
        Ok(())
    }

    #[test]
    fn a_creature_with_defender_cannot_be_declared_as_an_attacker() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.battlefield([WALL_OF_STONE]), P2]);
        let [wall] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(WALL_OF_STONE)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])?;

        game.act(p1, actions::Action::DeclareAttackers(vec![wall.into()]));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotAttack {
                attackers: vec![wall.into()],
                why: vec![actions::DeclarationWhy {
                    creature: Some(wall.into()),
                    why: rules::Why::Restriction(keywords::defender::CANT_ATTACK),
                }],
            })
        );
        assert!(!game.is_attacking(wall));
        Ok(())
    }

    #[test]
    fn a_creature_with_haste_can_attack_the_turn_it_entered() -> TestResult {
        use crate::HAND;
        use crate::mana;

        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([RAGING_GOBLIN, WALKING_CORPSE]).mana_pool(mana![1, R, B]),
            P2,
        ]);
        let [goblin] = game.player(p1).cards_in(HAND).exactly::<1>(RAGING_GOBLIN)?;
        let [corpse] = game.player(p1).cards_in(HAND).exactly::<1>(WALKING_CORPSE)?;
        game.play([p1.cast(goblin).pass_priority(), p2.pass_priority()])?;
        game.play([p1.cast(corpse).pass_priority(), p2.pass_priority()])?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])?;

        let [goblin_on_battlefield] =
            game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(RAGING_GOBLIN)?;
        let [corpse_on_battlefield] =
            game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(WALKING_CORPSE)?;
        game.act(
            p1,
            actions::Action::DeclareAttackers(vec![
                goblin_on_battlefield.into(),
                corpse_on_battlefield.into(),
            ]),
        );

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotAttack {
                attackers: vec![goblin_on_battlefield.into(), corpse_on_battlefield.into()],
                why: vec![actions::DeclarationWhy {
                    creature: Some(corpse_on_battlefield.into()),
                    why: rules::Why::Restriction(events::attack::SUMMONING_SICK),
                }],
            })
        );
        assert!(!game.is_attacking(goblin_on_battlefield));
        assert!(!game.is_attacking(corpse_on_battlefield));

        game.act(
            p1,
            actions::Action::DeclareAttackers(vec![goblin_on_battlefield.into()]),
        );

        assert!(game.moved(goblin).to(BATTLEFIELD).of(p1));
        assert!(game.is_attacking(goblin_on_battlefield));
        assert!(!game.is_attacking(corpse_on_battlefield));
        assert_eq!(game.rejection_count(), 1);
        Ok(())
    }

    #[test]
    fn a_creature_that_must_attack_cannot_be_left_out_of_the_declaration() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.battlefield([MASTER_OF_THE_FEAST, GRIZZLY_BEARS]), P2]);
        let [feast] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(MASTER_OF_THE_FEAST)?;
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])?;

        game.act(p1, actions::Action::DeclareAttackers(vec![]));
        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotAttack {
                attackers: vec![],
                why: vec![actions::DeclarationWhy {
                    creature: Some(feast.into()),
                    why: rules::Why::Required,
                }],
            })
        );
        assert!(!game.is_attacking(feast));

        game.act(p1, actions::Action::DeclareAttackers(vec![bears.into()]));
        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotAttack {
                attackers: vec![bears.into()],
                why: vec![actions::DeclarationWhy {
                    creature: Some(feast.into()),
                    why: rules::Why::Required,
                }],
            })
        );
        assert!(!game.is_attacking(bears));

        game.play([p1.declare_attackers([feast, bears])])?;
        assert!(game.is_attacking(feast));
        assert!(game.is_attacking(bears));
        Ok(())
    }

    #[test]
    fn a_static_haste_grant_allows_attacking_the_turn_it_entered() -> TestResult {
        use crate::HAND;
        use crate::mana;

        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TEST_HASTY_BANNER]).hand([WALKING_CORPSE]).mana_pool(mana![1, B]),
            P2,
        ]);
        let [corpse] = game.player(p1).cards_in(HAND).exactly::<1>(WALKING_CORPSE)?;
        game.play([p1.cast(corpse).pass_priority(), p2.pass_priority()])?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])?;

        let [corpse_on_battlefield] =
            game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(WALKING_CORPSE)?;
        game.act(
            p1,
            actions::Action::DeclareAttackers(vec![corpse_on_battlefield.into()]),
        );

        assert!(game.moved(corpse).to(BATTLEFIELD).of(p1));
        assert!(game.is_attacking(corpse_on_battlefield));
        assert_eq!(game.last_rejection(), None);
        Ok(())
    }

    #[test]
    fn a_static_defender_loss_allows_attacking() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([WALL_OF_STONE]),
            P2.battlefield([TEST_WALL_OF_DREAD]),
        ]);
        let [wall] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(WALL_OF_STONE)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([wall]),
        ])?;

        assert!(game.is_attacking(wall));
        Ok(())
    }

    #[test]
    fn a_static_attack_as_though_it_had_haste_allows_a_new_creature_to_attack() -> TestResult {
        use bevy::ecs::system::RunSystemOnce;

        use crate::HAND;
        use crate::keywords;
        use crate::mana;
        use crate::rules;

        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([TEST_BATTLE_FERVOR]).hand([WALKING_CORPSE]).mana_pool(mana![1, B]),
            P2,
        ]);
        let [corpse] = game.player(p1).cards_in(HAND).exactly::<1>(WALKING_CORPSE)?;
        game.play([p1.cast(corpse).pass_priority(), p2.pass_priority()])?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])?;

        let [corpse_on_battlefield] =
            game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(WALKING_CORPSE)?;
        game.act(
            p1,
            actions::Action::DeclareAttackers(vec![corpse_on_battlefield.into()]),
        );

        assert!(game.moved(corpse).to(BATTLEFIELD).of(p1));
        assert!(game.is_attacking(corpse_on_battlefield));
        assert_eq!(game.last_rejection(), None);
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(
                    !answers
                        .characteristics(corpse_on_battlefield.into())
                        .keywords
                        .contains(&keywords::haste())
                );
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn a_static_must_attack_grant_forces_an_attack() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.battlefield([TEST_WAR_DRUMS, GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])?;

        game.act(p1, actions::Action::DeclareAttackers(vec![]));
        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotAttack {
                attackers: vec![],
                why: vec![actions::DeclarationWhy {
                    creature: Some(bears.into()),
                    why: rules::Why::Required,
                }],
            })
        );
        assert!(!game.is_attacking(bears));

        game.play([p1.declare_attackers([bears])])?;
        assert!(game.is_attacking(bears));
        Ok(())
    }

    #[test]
    fn a_static_must_attack_rule_answer_forces_an_attack() -> TestResult {
        use bevy::ecs::system::RunSystemOnce;

        use crate::rules;

        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS]),
            P2.battlefield([TEST_CALL_TO_ARMS]),
        ]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])?;

        game.act(p1, actions::Action::DeclareAttackers(vec![]));
        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotAttack {
                attackers: vec![],
                why: vec![actions::DeclarationWhy {
                    creature: Some(bears.into()),
                    why: rules::Why::Required,
                }],
            })
        );
        assert!(!game.is_attacking(bears));

        game.play([p1.declare_attackers([bears])])?;
        assert!(game.is_attacking(bears));
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(answers.required::<events::Attack>(bears.into()));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn a_creature_that_waives_untapped_can_attack_while_tapped() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.battlefield_tapped([TEST_RESTLESS_WATCHER]), P2]);
        let [watcher] =
            game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TEST_RESTLESS_WATCHER)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([watcher]),
        ])?;

        assert!(game.is_attacking(watcher));
        Ok(())
    }

    #[test]
    fn a_creature_that_waives_defender_can_attack() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.battlefield([TEST_EAGER_WALL]), P2]);
        let [wall] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TEST_EAGER_WALL)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([wall]),
        ])?;

        assert!(game.is_attacking(wall));
        Ok(())
    }

    #[test]
    fn silent_arbiter_limits_the_whole_declaration_to_one_attacker() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS, GRIZZLY_BEARS]),
            P2.battlefield([TEST_SILENT_ARBITER]),
        ]);
        let [bears_1, bears_2] =
            game.player(p1).cards_in(BATTLEFIELD).exactly::<2>(GRIZZLY_BEARS)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])?;

        game.act(
            p1,
            actions::Action::DeclareAttackers(vec![bears_1.into(), bears_2.into()]),
        );
        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotAttack {
                attackers: vec![bears_1.into(), bears_2.into()],
                why: vec![actions::DeclarationWhy {
                    creature: None,
                    why: rules::Why::Restriction(rules::RestrictionId("test/silent-arbiter")),
                }],
            })
        );
        assert!(!game.is_attacking(bears_1));
        assert!(!game.is_attacking(bears_2));

        game.play([p1.declare_attackers([bears_1])])?;
        assert!(game.is_attacking(bears_1));
        assert!(!game.is_attacking(bears_2));
        Ok(())
    }
}
