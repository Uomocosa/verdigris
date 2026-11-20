use bevy::prelude::{Commands, Entity, MessageReader, MessageWriter, Query, Res};

use crate::BATTLEFIELD;
use crate::actions;
use crate::cards;
use crate::rules::helpers;
use crate::turns;
use crate::zones;

/// 508.1a The active player chooses which creatures that they control, if any, will attack. The
/// chosen creatures must be untapped, they can’t also be battles, and each one must either have
/// haste or have been controlled by the active player continuously since the turn began.
///
/// 508.1k Each chosen creature still controlled by the active player becomes an attacking
/// creature. It remains an attacking creature until it’s removed from combat or the combat phase
/// ends, whichever comes first. See rule 506.4.
pub fn r508_1a_declare_attackers(
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
    zone_entities: Query<(Entity, &zones::Zone, Option<&cards::Owner>)>,
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
        let Some(battlefield) =
            helpers::find_zone(BATTLEFIELD, Some(action.player), &zone_entities)
        else {
            rejected.write(actions::Rejected::CannotAttack {
                attackers: attackers.clone(),
            });
            continue;
        };
        let all_legal = attackers.iter().all(|attacker| {
            cards
                .get(*attacker)
                .is_ok_and(|(definition, owner, in_zone, tapped)| {
                    definition.types.contains(&cards::CardType::Creature)
                        && **owner == action.player
                        && **in_zone == battlefield
                        && tapped.is_none()
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
    fn test_usage() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])
        .unwrap();
        let [bears] = game
            .player(p1)
            .cards_in(BATTLEFIELD)
            .matching(GRIZZLY_BEARS)
            .exactly::<1>()
            .unwrap();

        game.act(p1, actions::Action::DeclareAttackers(vec![bears.into()]));

        assert!(
            game.app
                .world()
                .get::<crate::cards::Attacking>(bears.into())
                .is_some()
        );
    }

    #[test]
    fn r508_1a_declared_creatures_become_attacking() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])
        .unwrap();
        let [attacking] = game
            .player(p1)
            .cards_in(BATTLEFIELD)
            .matching(GRIZZLY_BEARS)
            .exactly::<1>()
            .unwrap();
        let [defending] = game
            .player(p2)
            .cards_in(BATTLEFIELD)
            .matching(GRIZZLY_BEARS)
            .exactly::<1>()
            .unwrap();

        game.act(
            p1,
            actions::Action::DeclareAttackers(vec![attacking.into()]),
        );

        assert!(
            game.app
                .world()
                .get::<crate::cards::Attacking>(attacking.into())
                .is_some()
        );
        assert!(
            game.app
                .world()
                .get::<crate::cards::Attacking>(defending.into())
                .is_none()
        );
    }

    #[test]
    fn cannot_declare_another_players_creature() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])
        .unwrap();
        let [defending] = game
            .player(p2)
            .cards_in(BATTLEFIELD)
            .matching(GRIZZLY_BEARS)
            .exactly::<1>()
            .unwrap();

        game.act(
            p1,
            actions::Action::DeclareAttackers(vec![defending.into()]),
        );

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotAttack {
                attackers: vec![defending.into()]
            })
        );
        assert!(
            game.app
                .world()
                .get::<crate::cards::Attacking>(defending.into())
                .is_none()
        );
    }

    #[test]
    fn r508_1a_a_tapped_creature_cannot_be_declared_as_an_attacker() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield_tapped([GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])
        .unwrap();
        let [tapped] = game
            .player(p1)
            .cards_in(BATTLEFIELD)
            .matching(GRIZZLY_BEARS)
            .exactly::<1>()
            .unwrap();

        game.act(p1, actions::Action::DeclareAttackers(vec![tapped.into()]));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotAttack {
                attackers: vec![tapped.into()]
            })
        );
        assert!(
            game.app
                .world()
                .get::<crate::cards::Attacking>(tapped.into())
                .is_none()
        );
    }
}
