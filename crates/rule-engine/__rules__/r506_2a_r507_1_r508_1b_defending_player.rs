use bevy::prelude::{Commands, Entity, MessageReader, MessageWriter, Query, Res, With};

use crate::actions;
use crate::players;
use crate::turns;

/// 506.2a During the combat phase of a multiplayer game, there may be one or more defending
/// players, depending on the variant being played and the options chosen for it. Unless all the
/// attacking player’s opponents automatically become defending players during the combat phase,
/// the attacking player chooses one of their opponents as a turn-based action during the
/// beginning of combat step. (Note that the choice may be dictated by the variant being played or
/// the options chosen for it.) That player becomes the defending player. See rule 802, “Attack
/// Multiple Players Option,” rule 803, “Attack Left and Attack Right Options,” and rule 809,
/// “Emperor Variant.”
///
/// 507.1. First, if the game being played is a multiplayer game in which the active player’s
/// opponents don’t all automatically become defending players, the active player chooses one of
/// their opponents. That player becomes the defending player. This turn-based action doesn’t use
/// the stack. (See rule 506.2.)
///
/// 508.1b If the defending player controls any planeswalkers, is the protector of any battles,
/// or the game allows the active player to attack multiple other players, the active player
/// announces which player, planeswalker, or battle each of the chosen creatures is attacking.
pub fn r506_2a_r507_1_r508_1b_defending_player(
    mut step_started: MessageReader<turns::StepStarted>,
    mut actions: MessageReader<actions::PlayerAction>,
    mut rejected: MessageWriter<actions::Rejected>,
    mut commands: Commands,
    turn: Res<turns::Turn>,
    priority: Res<turns::Priority>,
    players: Query<Entity, With<players::Player>>,
    defending: Query<Entity, With<players::DefendingPlayer>>,
    lost: Query<Entity, With<players::Lost>>,
) {
    let turn = turn.into_inner();
    let priority = priority.into_inner();
    for step in step_started.read() {
        if **step != turns::Step::BeginningOfCombat {
            continue;
        }
        for player in defending.iter() {
            commands.entity(player).remove::<players::DefendingPlayer>();
        }
        let mut opponents = players.iter().filter(|player| *player != turn.active);
        if let (Some(solo_opponent), None) = (opponents.next(), opponents.next()) {
            commands.entity(solo_opponent).insert(players::DefendingPlayer);
        }
    }
    for action in actions.read() {
        let actions::Action::ChooseDefendingPlayer(chosen) = &action.action else {
            continue;
        };
        let is_legal = turn.step == turns::Step::BeginningOfCombat
            && action.player == turn.active
            && action.player == priority.player
            && players.get(*chosen).is_ok()
            && *chosen != turn.active
            && lost.get(*chosen).is_err();
        if !is_legal {
            rejected.write(actions::Rejected::CannotChooseDefendingPlayer {
                player: action.player,
            });
            continue;
        }
        for player in defending.iter() {
            commands.entity(player).remove::<players::DefendingPlayer>();
        }
        commands.entity(*chosen).insert(players::DefendingPlayer);
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::BATTLEFIELD;
    use crate::actions;
    use crate::players;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn r507_1_the_active_player_chooses_one_opponent_as_the_defending_player() -> TestResult {
        let (mut game, [p1, p2, p3]) =
            MockGame::builder().turn(2, turns::Step::BeginningOfCombat).players([
                P1,
                P2.active().battlefield([GRIZZLY_BEARS]),
                P3.battlefield([GRIZZLY_BEARS]),
            ]);
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;

        game.act(p2, actions::Action::ChooseDefendingPlayer(p3.into()));
        game.play([
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.pass_to(turns::Step::DeclareAttackers),
            p3.pass_to(turns::Step::DeclareAttackers),
            p2.declare_attackers([bears]),
        ])?;
        game.play([
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_to(turns::Step::CombatDamage),
            p3.pass_to(turns::Step::CombatDamage),
        ])?;
        game.play([p2.pass_priority(), p3.pass_priority(), p1.pass_priority()])?;

        assert_eq!(game.life(p3), 18);
        assert_eq!(game.life(p1), 20);
        Ok(())
    }

    #[test]
    fn r507_1_the_defending_player_marker_is_cleared_at_the_next_beginning_of_combat() -> TestResult
    {
        let (mut game, [p1, p2, p3]) = MockGame::new([P1, P2, P3]);
        game.play([
            p1.pass_to(turns::Step::BeginningOfCombat),
            p2.pass_to(turns::Step::BeginningOfCombat),
            p3.pass_to(turns::Step::BeginningOfCombat),
        ])?;

        game.act(p1, actions::Action::ChooseDefendingPlayer(p3.into()));
        let marker: Option<Entity> =
            game.app.world().get::<players::DefendingPlayer>(p3.into()).map(|_| p3.into());
        assert_eq!(marker, Some(p3.into()));

        game.play([
            p1.pass_to(turns::Step::BeginningOfCombat),
            p2.pass_to(turns::Step::BeginningOfCombat),
            p3.pass_to(turns::Step::BeginningOfCombat),
        ])?;

        assert!(game.app.world().get::<players::DefendingPlayer>(p1.into()).is_none());
        assert!(game.app.world().get::<players::DefendingPlayer>(p2.into()).is_none());
        assert!(game.app.world().get::<players::DefendingPlayer>(p3.into()).is_none());
        Ok(())
    }

    #[test]
    fn r507_1_a_player_who_has_lost_cannot_be_chosen_as_the_defending_player() -> TestResult {
        let (mut game, [p1, p2, p3]) = MockGame::new([P1, P2, P3.life(0)]);
        game.play([
            p1.pass_to(turns::Step::BeginningOfCombat),
            p2.pass_to(turns::Step::BeginningOfCombat),
            p3.pass_to(turns::Step::BeginningOfCombat),
        ])?;
        assert!(game.app.world().get::<players::Lost>(p3.into()).is_some());

        game.act(p1, actions::Action::ChooseDefendingPlayer(p3.into()));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotChooseDefendingPlayer { player: *p1 })
        );
        assert!(game.app.world().get::<players::DefendingPlayer>(p3.into()).is_none());
        Ok(())
    }

    #[test]
    fn test_usage() -> TestResult {
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
        Ok(())
    }
}
