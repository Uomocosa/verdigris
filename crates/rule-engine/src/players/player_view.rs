use crate::cards;
use crate::players;
use atomic_delegate_macros::atomic_delegates;

#[derive(Clone, Copy)]
pub struct PlayerView<'w, 's> {
    pub id: players::PlayerId,
    pub(crate) players: players::Players<'w, 's>,
}

#[rustfmt::skip]
impl PlayerView<'_, '_> {
    #[must_use]
    pub fn opponents_still_in_the_game(&self) -> Vec<Self> { self.opponents().into_iter().filter(PlayerView::is_still_in_the_game).collect() }

    #[must_use]
    pub fn is_still_in_the_game(&self) -> bool { self.players.is_still_in_the_game(self.id) }

    #[must_use]
    pub fn has_lost(&self) -> bool { self.players.has_lost(self.id) }

    #[must_use]
    pub fn have_all_opponents_left(&self) -> bool { !self.opponents().is_empty() && self.opponents_still_in_the_game().is_empty() }
}

#[atomic_delegates]
impl PlayerView<'_, '_> {
    #[must_use]
    pub fn opponents(&self) -> Vec<Self> {}

    #[must_use]
    pub fn next_in_turn_order(&self) -> Option<Self> {}

    #[must_use]
    pub fn opening_hand(&self) -> Vec<cards::CardId> {}
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::players;
    use crate::testing::*;

    use super::PlayerView;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, _p2, _p3]) = MockGame::new([P1, P2, P3.life(0)]);
        game.play([p1.pass_priority()])?;
        game.app
            .world_mut()
            .run_system_once(move |all: players::Players| {
                let view: PlayerView = all.view(p1.id);
                assert_eq!(view.id, p1.id);
                assert!(view.is_still_in_the_game());
                assert!(!view.have_all_opponents_left());
            })
            .unwrap();
        Ok(())
    }
}
