use bevy::prelude::{Commands, MessageReader, MessageWriter, Query, Res};

use crate::BATTLEFIELD;
use crate::actions;
use crate::cards;
use crate::keywords;
use crate::turns;
use crate::zones;

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
    cards: Query<(
        &cards::CardDefinition,
        &cards::Owner,
        &zones::InZone,
        Option<&cards::Tapped>,
    )>,
    entered_on: Query<Option<&cards::EnteredOnTurn>>,
    zones: zones::ZoneLookup,
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
        let Some(battlefield) = zones.find(BATTLEFIELD, Some(action.player)) else {
            rejected.write(actions::Rejected::CannotAttack {
                attackers: attackers.clone(),
            });
            continue;
        };
        let all_legal = attackers.iter().all(|attacker| {
            cards.get(*attacker).is_ok_and(|(definition, owner, in_zone, tapped)| {
                definition.types.contains(&cards::CardType::Creature)
                    && **owner == action.player
                    && **in_zone == battlefield
                    && tapped.is_none()
                    && !definition.keywords.contains(&keywords::Keyword::Defender)
                    && (definition.keywords.contains(&keywords::Keyword::Haste)
                        || entered_on.get(*attacker).is_ok_and(|on_turn| {
                            on_turn.is_none_or(|turn_number| **turn_number != turn.number)
                        }))
            })
        });
        if !all_legal {
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
    fn cannot_declare_another_players_creature() -> TestResult {
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
}
