use bevy::ecs::system::SystemParam;
use bevy::prelude::Query;

use crate::players;
use crate::turns;

#[derive(SystemParam, Clone, Copy)]
pub struct Game<'w, 's> {
    players: players::Players<'w, 's>,
    turn: Query<'w, 's, &'static turns::Turn>,
    priority: Query<'w, 's, &'static turns::Priority>,
}

impl<'w, 's> Game<'w, 's> {
    #[must_use]
    pub fn priority_holder(&self) -> Option<players::PlayerView<'w, 's>> {
        let holder = self.priority.single().ok()?.holder;
        Some(self.players.view(holder))
    }

    #[must_use]
    pub fn active_player(&self) -> Option<players::PlayerView<'w, 's>> {
        let active = self.turn.single().ok()?.active;
        if self.players.is_still_in_the_game(active) {
            Some(self.players.view(active))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::game;
    use crate::testing::*;

    #[test]
    fn test_usage() {
        let (mut game, [p1, _p2]) = MockGame::new([P1, P2]);
        game.app
            .world_mut()
            .run_system_once(move |answers: game::Game| {
                assert_eq!(answers.active_player().map(|player| player.id), Some(p1.id));
                assert_eq!(
                    answers.priority_holder().map(|player| player.id),
                    Some(p1.id)
                );
            })
            .unwrap();
    }

    #[test]
    fn active_player_is_none_when_the_active_player_lost() -> TestResult {
        let (mut game, [p1, _p2, _p3]) = MockGame::new([P1.life(0), P2, P3]);
        game.play([p1.pass_priority()])?;
        game.app
            .world_mut()
            .run_system_once(move |answers: game::Game| {
                assert!(answers.active_player().is_none());
            })
            .unwrap();
        Ok(())
    }
}
