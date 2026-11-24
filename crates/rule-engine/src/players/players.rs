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
    lost: Query<'w, 's, Entity, With<players::Lost>>,
}

#[rustfmt::skip]
impl Players<'_, '_> {
    #[must_use]
    pub fn is_still_in_the_game(&self, player: players::PlayerId) -> bool { self.still_in_the_game.contains(*player) }

    #[must_use]
    pub fn has_lost(&self, player: players::PlayerId) -> bool { self.lost.contains(*player) }

    #[must_use]
    pub fn has_anyone_won(&self) -> bool { !self.winners.is_empty() }

    #[must_use]
    pub fn in_turn_order(&self) -> Vec<players::PlayerId> { self.turn_order.iter().flat_map(|order| order.iter().copied()).collect() }
}

#[atomic_delegates]
impl Players<'_, '_> {
    #[must_use]
    pub fn still_in_the_game(&self) -> Vec<players::PlayerView<'_, '_>> {}
}

#[rustfmt::skip]
impl<'w, 's> Players<'w, 's> {
    #[must_use]
    pub(crate) const fn view(self, id: players::PlayerId) -> players::PlayerView<'w, 's> { players::PlayerView { id, players: self } }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::testing::*;

    use super::Players;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2, _p3]) = MockGame::new([P1, P2.life(0), P3]);
        game.play([p1.pass_priority()])?;
        game.app
            .world_mut()
            .run_system_once(move |players: Players| {
                assert!(!players.is_still_in_the_game(p2.id));
                assert!(!players.has_anyone_won());
            })
            .unwrap();
        Ok(())
    }
}
