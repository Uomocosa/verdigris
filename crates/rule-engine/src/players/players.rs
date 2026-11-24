use atomic_delegate_macros::atomic_delegates;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Entity, Query, With};

use crate::players;
use crate::turns;

#[derive(SystemParam, Clone, Copy)]
pub struct Players<'w, 's> {
    pub(crate) turn_order: Query<'w, 's, &'static turns::TurnOrder>,
    still_in_the_game: Query<'w, 's, Entity, players::StillInTheGame>,
    winners: Query<'w, 's, Entity, With<players::Won>>,
}

#[rustfmt::skip]
impl Players<'_, '_> {
    #[must_use]
    pub fn opponents_of(&self, player: players::PlayerId) -> Vec<players::PlayerId> { self.in_turn_order().into_iter().filter(|other| *other != player).collect() }

    #[must_use]
    pub fn is_still_in_the_game(&self, player: players::PlayerId) -> bool { self.still_in_the_game.contains(*player) }

    #[must_use]
    pub fn has_anyone_won(&self) -> bool { !self.winners.is_empty() }
}

#[atomic_delegates]
impl Players<'_, '_> {
    #[must_use]
    pub fn in_turn_order(&self) -> Vec<players::PlayerId> {}

    #[must_use]
    pub fn still_in_the_game(&self) -> Vec<players::PlayerId> {}
}

impl Players<'_, '_> {
    #[must_use]
    pub fn have_all_opponents_left(&self, player: players::PlayerId) -> bool {
        let opponents = self.opponents_of(player);
        let is_any_still_here = opponents.iter().any(|other| self.is_still_in_the_game(*other));
        !opponents.is_empty() && !is_any_still_here
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::testing::*;

    use super::Players;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2, p3]) = MockGame::new([P1, P2.life(0), P3]);
        game.play([p1.pass_priority()])?;
        game.app
            .world_mut()
            .run_system_once(move |players: Players| {
                assert_eq!(players.opponents_of(p1.id), [p2.id, p3.id]);
                assert!(!players.is_still_in_the_game(p2.id));
                assert!(!players.have_all_opponents_left(p1.id));
                assert!(!players.has_anyone_won());
            })
            .unwrap();
        Ok(())
    }
}
