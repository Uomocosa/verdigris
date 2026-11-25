use bevy::prelude::{Commands, Entity, MessageReader, MessageWriter, Query, With};

use crate::Game;
use crate::actions;
use crate::players;
use crate::turns;

/// 507.1. First, if the game being played is a multiplayer game in which the active player’s
/// opponents don’t all automatically become defending players, the active player chooses one of
/// their opponents. That player becomes the defending player. This turn-based action doesn’t use
/// the stack. (See rule 506.2.)
///
/// # See also
///
/// 506.2a During the combat phase of a multiplayer game, there may be one or more defending
/// players, depending on the variant being played and the options chosen for it. Unless all the
/// attacking player’s opponents automatically become defending players during the combat phase,
/// the attacking player chooses one of their opponents as a turn-based action during the
/// beginning of combat step. (Note that the choice may be dictated by the variant being played or
/// the options chosen for it.) That player becomes the defending player. See rule 802, “Attack
/// Multiple Players Option,” rule 803, “Attack Left and Attack Right Options,” and rule 809,
/// “Emperor Variant.”
pub fn r507_1_choose_defending_player(
    mut actions: MessageReader<actions::PlayerAction>,
    mut rejected: MessageWriter<actions::Rejected>,
    mut commands: Commands,
    game: Game,
    defending: Query<Entity, With<players::DefendingPlayer>>,
) {
    for action in actions.read() {
        let actions::Action::ChooseDefendingPlayer(chosen) = &action.action else {
            continue;
        };
        let is_legal = game.turn().step() == Some(turns::Step::BeginningOfCombat)
            && game.active_player().is_some_and(|attacking_player| {
                action.player == attacking_player.id
                    && game.priority_holder().is_some_and(|holder| action.player == holder.id)
                    && attacking_player
                        .opponents_still_in_the_game()
                        .iter()
                        .any(|opponent| opponent.id == *chosen)
            });
        if !is_legal {
            rejected.write(actions::Rejected::CannotChooseDefendingPlayer {
                player: action.player,
            });
            continue;
        }
        for player in defending.iter() {
            commands.entity(player).remove::<players::DefendingPlayer>();
        }
        commands.entity(**chosen).insert(players::DefendingPlayer);
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::actions;
    use crate::players;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn the_active_player_chooses_one_opponent_as_the_defending_player() -> TestResult {
        let (mut game, [p1, p2, p3]) =
            MockGame::builder().turn(2, turns::Step::BeginningOfCombat).players([
                P1,
                P2.active().battlefield([GRIZZLY_BEARS]),
                P3.battlefield([GRIZZLY_BEARS]),
            ]);
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;

        game.act(p2, actions::Action::ChooseDefendingPlayer(p3.id));
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
    fn a_player_who_has_lost_cannot_be_chosen_as_the_defending_player() -> TestResult {
        let (mut game, [p1, p2, p3]) = MockGame::new([P1, P2, P3.life(0)]);
        game.play([
            p1.pass_to(turns::Step::BeginningOfCombat),
            p2.pass_to(turns::Step::BeginningOfCombat),
            p3.pass_to(turns::Step::BeginningOfCombat),
        ])?;
        assert!(game.has_lost(p3));

        game.act(p1, actions::Action::ChooseDefendingPlayer(p3.id));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotChooseDefendingPlayer { player: p1.id })
        );
        assert!(!game.has::<players::DefendingPlayer>(p3));
        Ok(())
    }
}
