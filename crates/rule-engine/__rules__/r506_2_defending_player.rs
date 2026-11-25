use bevy::ecs::schedule::{IntoScheduleConfigs, ScheduleConfigs};
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::{Commands, Entity, Query, With};

use crate::Game;
use crate::players;
use crate::turns;

/// 506.2. During the combat phase, the active player is the attacking player; creatures that
/// player controls may attack. During the combat phase of a two-player game, the nonactive player
/// is the defending player; that player, planeswalkers they control, and battles they protect may
/// be attacked.
pub fn r506_2_defending_player() -> ScheduleConfigs<ScheduleSystem> {
    only_opponent_is_the_defending_player.run_if(turns::Step::BeginningOfCombat.begins())
}

fn only_opponent_is_the_defending_player(
    mut commands: Commands,
    game: Game,
    defending: Query<Entity, With<players::DefendingPlayer>>,
) {
    for player in defending.iter() {
        commands.entity(player).remove::<players::DefendingPlayer>();
    }
    if let Some(attacking_player) = game.active_player()
        && let [only_opponent] = attacking_player.opponents_still_in_the_game().as_slice()
    {
        commands.entity(*only_opponent.id).insert(players::DefendingPlayer);
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::players;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn the_defending_player_marker_is_cleared_at_the_next_beginning_of_combat() -> TestResult {
        let (mut game, [p1, p2, p3]) = MockGame::new([P1, P2, P3]);
        game.play([
            p1.pass_to(turns::Step::BeginningOfCombat),
            p2.pass_to(turns::Step::BeginningOfCombat),
            p3.pass_to(turns::Step::BeginningOfCombat),
        ])?;

        game.play([p1.choose_defending_player(p3)])?;
        let marker: Option<Entity> = game.get::<players::DefendingPlayer>(p3).map(|_| p3.into());
        assert_eq!(marker, Some(p3.into()));

        game.play([
            p1.pass_to(turns::Step::BeginningOfCombat),
            p2.pass_to(turns::Step::BeginningOfCombat),
            p3.pass_to(turns::Step::BeginningOfCombat),
        ])?;

        assert!(!game.has::<players::DefendingPlayer>(p1));
        assert!(!game.has::<players::DefendingPlayer>(p2));
        assert!(!game.has::<players::DefendingPlayer>(p3));
        Ok(())
    }

    #[test]
    fn sole_opponent_automatically_becomes_defending_player() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1, P2]);
        game.play([
            p1.pass_to(turns::Step::BeginningOfCombat),
            p2.pass_to(turns::Step::BeginningOfCombat),
        ])?;

        assert!(game.has::<players::DefendingPlayer>(p2));
        assert!(!game.has::<players::DefendingPlayer>(p1));
        Ok(())
    }

    #[test]
    fn the_only_opponent_still_in_the_game_becomes_the_defending_player() -> TestResult {
        let (mut game, [p1, p2, p3]) = MockGame::new([P1, P2, P3.life(0)]);
        game.play([
            p1.pass_to(turns::Step::BeginningOfCombat),
            p2.pass_to(turns::Step::BeginningOfCombat),
            p3.pass_to(turns::Step::BeginningOfCombat),
        ])?;
        assert!(game.has_lost(p3));
        assert!(game.has::<players::DefendingPlayer>(p2));
        assert!(!game.has::<players::DefendingPlayer>(p1));
        assert!(!game.has::<players::DefendingPlayer>(p3));
        Ok(())
    }
}
