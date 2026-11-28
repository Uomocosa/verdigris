use bevy::ecs::schedule::{IntoScheduleConfigs, ScheduleConfigs};
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::{Commands, Entity, Query, With};

use crate::GameMut;
use crate::cards;
use crate::players;
use crate::zones;

/// 800.4a When a player leaves the game, all objects (see rule 109) owned by that player leave the
/// game and any effects which give that player control of any objects or players end. Then, if that
/// player controlled any objects on the stack not represented by cards, those objects cease to
/// exist. Then, if there are any objects still controlled by that player, those objects are exiled.
/// This is not a state-based action. It happens as soon as the player leaves the game. If the
/// player who left the game had priority at the time they left, priority passes to the next player
/// in turn order who’s still in the game.
///
/// # Notes
///
/// Implemented here: all objects owned by a player who left leave the game, and priority passes to
/// the next player in turn order who’s still in the game. Not implemented: “any effects which give
/// that player control of any objects or players end”; “Then, if that player controlled any objects
/// on the stack not represented by cards, those objects cease to exist”; “Then, if there are any
/// objects still controlled by that player, those objects are exiled.”
pub fn r800_4a_priority_of_a_player_who_left() -> ScheduleConfigs<ScheduleSystem> {
    (
        objects_owned_by_players_who_left,
        priority_passes_to_the_next_player_still_in_the_game,
    )
        .chain()
}

fn objects_owned_by_players_who_left(
    mut commands: Commands,
    players: players::Players,
    objects: Query<(Entity, &cards::Owner), With<zones::InZone>>,
) {
    let players_who_left = players.players_who_left();
    for (object, owner) in &objects {
        if players_who_left.contains(&**owner) {
            commands.entity(object).despawn();
        }
    }
}

fn priority_passes_to_the_next_player_still_in_the_game(mut game: GameMut) {
    let Some(holder) = game.priority_holder() else {
        return;
    };
    if holder.is_still_in_the_game() {
        return;
    }
    if let Some(next) = holder.next_in_turn_order() {
        game.pass_priority_to(next.id);
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;
    use crate::{BATTLEFIELD, EXILE, GRAVEYARD, HAND, LIBRARY};

    #[test]
    fn priority_passes_on_when_its_holder_concedes() -> TestResult {
        let (mut game, [p1, p2, p3]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2, P3]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;

        game.play([p1.concede()])?;

        assert_eq!(game.priority_player(), Some(p2.id));
        for zone in [BATTLEFIELD, HAND, LIBRARY, GRAVEYARD, EXILE] {
            for player in [p1, p2, p3] {
                assert!(!game.card(bears).is_in(zone).of(player));
            }
        }
        Ok(())
    }

    #[test]
    fn priority_passes_on_when_its_holder_loses_to_a_state_based_action() -> TestResult {
        let (mut game, [p1, _p2, p3]) = MockGame::new([P1, P2.life(0), P3]);

        game.play([p1.pass_priority()])?;

        assert_eq!(game.priority_player(), Some(p3.id));
        Ok(())
    }
}
