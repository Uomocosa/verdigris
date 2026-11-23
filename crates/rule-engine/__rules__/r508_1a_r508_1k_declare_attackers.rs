use bevy::prelude::{Commands, Entity, MessageReader, MessageWriter, Query, Res, With};

use crate::actions;
use crate::cards;
use crate::rules;
use crate::turns;

/// 508.1a The active player chooses which creatures that they control, if any, will attack. The
/// chosen creatures must be untapped, they can’t also be battles, and each one must either have
/// haste or have been controlled by the active player continuously since the turn began.
///
/// 508.1k Each chosen creature still controlled by the active player becomes an attacking creature.
/// It remains an attacking creature until it’s removed from combat or the combat phase ends,
/// whichever comes first. See rule 506.4.
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
pub fn r508_1a_r508_1k_declare_attackers(
    mut actions: MessageReader<actions::PlayerAction>,
    mut rejected: MessageWriter<actions::Rejected>,
    mut commands: Commands,
    priority: Res<turns::Priority>,
    turn: Res<turns::Turn>,
    candidates: Query<(Entity, &cards::Owner), With<cards::Creature>>,
    answers: rules::Rules,
) {
    let priority = priority.into_inner();
    let turn = turn.into_inner();
    for action in actions.read() {
        let actions::Action::DeclareAttackers(attackers) = &action.action else {
            continue;
        };
        if turn.step != turns::Step::DeclareAttackers
            || action.player != priority.player
            || action.player != turn.active
        {
            rejected.write(actions::Rejected::CannotAttack {
                attackers: attackers.clone(),
            });
            continue;
        }
        let all_legal = attackers.iter().all(|attacker| {
            answers.legal::<rules::DeclareAttacker>(&rules::AttackCtx {
                attacker: *attacker,
                player: action.player,
            })
        });
        let requirement_unmet = candidates.iter().any(|(entity, owner)| {
            **owner == action.player
                && answers.demanded::<rules::DeclareAttacker>(entity)
                && answers.legal::<rules::DeclareAttacker>(&rules::AttackCtx {
                    attacker: entity,
                    player: action.player,
                })
                && !attackers.contains(&entity)
        });
        if !all_legal || requirement_unmet {
            rejected.write(actions::Rejected::CannotAttack {
                attackers: attackers.clone(),
            });
            continue;
        }
        for attacker in attackers {
            commands.entity(*attacker).insert(cards::Attacking);
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
    fn declared_creatures_become_attacking() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [bears_1] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [bears_2] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([bears_1]),
        ])?;

        assert!(game.is_attacking(bears_1));
        assert!(!game.is_attacking(bears_2));
        Ok(())
    }

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
                attackers: vec![bears.into()]
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
                attackers: vec![tundra.into()]
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

        game.act(p1, actions::Action::DeclareAttackers(vec![bears.into()]));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotAttack {
                attackers: vec![bears.into()]
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
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])?;

        game.act(p1, actions::Action::DeclareAttackers(vec![corpse.into()]));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotAttack {
                attackers: vec![corpse.into()]
            })
        );
        assert!(!game.is_attacking(corpse));
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
                attackers: vec![wall.into()]
            })
        );
        assert!(!game.is_attacking(wall));
        Ok(())
    }

    #[test]
    fn a_creature_with_haste_can_attack_the_turn_it_entered() -> TestResult {
        use crate::HAND;
        use crate::mana;

        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([RAGING_GOBLIN]).mana_pool(mana![R]), P2]);
        let [goblin] = game.player(p1).cards_in(HAND).exactly::<1>(RAGING_GOBLIN)?;
        game.play([p1.cast(goblin).pass_priority(), p2.pass_priority()])?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])?;

        game.act(p1, actions::Action::DeclareAttackers(vec![goblin.into()]));

        assert!(game.player(p1).cards_in(BATTLEFIELD).has(goblin));
        assert!(game.is_attacking(goblin));
        assert_eq!(game.last_rejection(), None);
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
            Some(&actions::Rejected::CannotAttack { attackers: vec![] })
        );
        assert!(!game.is_attacking(feast));

        game.act(p1, actions::Action::DeclareAttackers(vec![bears.into()]));
        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotAttack {
                attackers: vec![bears.into()]
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

        game.act(p1, actions::Action::DeclareAttackers(vec![corpse.into()]));

        assert!(game.player(p1).cards_in(BATTLEFIELD).has(corpse));
        assert!(game.is_attacking(corpse));
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

        game.act(p1, actions::Action::DeclareAttackers(vec![corpse.into()]));

        assert!(game.player(p1).cards_in(BATTLEFIELD).has(corpse));
        assert!(game.is_attacking(corpse));
        assert_eq!(game.last_rejection(), None);
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(
                    !answers
                        .characteristics(corpse.into())
                        .keywords
                        .contains(&keywords::StaticKeyword::Haste)
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
            Some(&actions::Rejected::CannotAttack { attackers: vec![] })
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
            Some(&actions::Rejected::CannotAttack { attackers: vec![] })
        );
        assert!(!game.is_attacking(bears));

        game.play([p1.declare_attackers([bears])])?;
        assert!(game.is_attacking(bears));
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(answers.demanded::<rules::DeclareAttacker>(bears.into()));
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
}
