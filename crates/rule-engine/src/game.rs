use bevy::ecs::system::SystemParam;
use bevy::prelude::Query;

use crate::players;
use crate::turns;

#[derive(SystemParam, Clone, Copy)]
pub struct Game<'w, 's> {
    players: players::Players<'w, 's>,
    state: Query<'w, 's, &'static turns::TurnState>,
}

#[rustfmt::skip]
impl Game<'_, '_> {
    #[must_use]
    pub fn is_a_draw(&self) -> bool { self.players.is_a_draw_for_anyone() && self.players.still_in_the_game().is_empty() }

    #[must_use]
    pub fn is_over(&self) -> bool { self.players.has_anyone_won() || self.is_a_draw() }
}

impl Game<'_, '_> {
    #[must_use]
    pub fn turn(&self) -> turns::TurnView<'_> {
        let Ok(state) = self.state.single_inner() else {
            return turns::TurnView(None);
        };
        turns::TurnView(Some(state))
    }
}

impl Game<'_, '_> {
    #[must_use]
    pub fn priority_holder(&self) -> Option<players::PlayerView<'_, '_>> {
        let holder = self.state.single_inner().ok()?.priority.holder?;
        Some(self.players.view(holder))
    }

    #[must_use]
    pub fn active_player(&self) -> Option<players::PlayerView<'_, '_>> {
        let active = self.state.single_inner().ok()?.turn.active;
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

    use crate::Game;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn test_usage() {
        let (mut game, [p1, _p2]) = MockGame::new([P1, P2]);
        game.app.update();
        game.app
            .world_mut()
            .run_system_once(move |answers: Game| {
                assert_eq!(answers.active_player().map(|player| player.id), Some(p1.id));
                assert_eq!(answers.turn().step(), Some(turns::Step::PrecombatMain));
                assert!(answers.turn().is_first_turn());
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
            .run_system_once(move |answers: Game| {
                assert!(answers.active_player().is_none());
            })
            .unwrap();
        Ok(())
    }
}
