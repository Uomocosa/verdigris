use atomic_delegate_macros::atomic_delegates;
use bevy::ecs::system::SystemParam;
use bevy::prelude::ResMut;

use crate::players;
use crate::turns;

#[derive(SystemParam)]
pub struct GameMut<'w, 's> {
    pub(crate) players: players::Players<'w, 's>,
    pub(crate) state: ResMut<'w, turns::TurnState>,
}

#[rustfmt::skip]
impl<'w, 's> GameMut<'w, 's> {
    #[must_use]
    pub fn step(&self) -> turns::Step { self.state.turn.step }

    #[must_use]
    pub fn priority_holder(&self) -> Option<players::PlayerView<'w, 's>> {
        self.state.priority.holder.map(|holder| self.players.view(holder))
    }

    pub fn give_priority_to(&mut self, player: players::PlayerId) { self.state.priority.give_to(player); }

    pub fn pass_priority_to(&mut self, next: players::PlayerId) { self.state.priority.pass_to(next); }

    pub fn add_step_after_this_one(&mut self, step: turns::Step) { self.state.add_step_after_this_one(step); }

    pub fn begin_step(&mut self, step: turns::Step) { self.state.begin_step(step); }
}

impl GameMut<'_, '_> {
    #[must_use]
    pub fn have_all_players_passed_in_succession(&self) -> bool {
        let still_in_the_game = self.players.still_in_the_game();
        !still_in_the_game.is_empty()
            && still_in_the_game.iter().all(|player| self.state.priority.has_passed(player.id))
    }

    pub fn give_priority_to_active_player(&mut self) {
        let active = self.state.turn.active;
        if self.players.is_still_in_the_game(active) {
            self.state.priority.give_to(active);
        } else if let Some(next) = self.players.view(active).next_in_turn_order() {
            self.state.priority.give_to(next.id);
        }
    }
}

#[atomic_delegates]
impl GameMut<'_, '_> {
    pub fn begin_next_turn(&mut self) -> Option<players::PlayerView<'_, '_>> {}
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::testing::*;
    use crate::turns;

    use super::GameMut;

    #[test]
    fn test_usage() {
        let (mut game, [p1, p2]) = MockGame::new([P1, P2]);
        game.app.update();
        game.app
            .world_mut()
            .run_system_once(move |mut answers: GameMut| {
                assert_eq!(answers.step(), turns::Step::PrecombatMain);
                assert_eq!(
                    answers.priority_holder().map(|player| player.id),
                    Some(p1.id)
                );
                answers.pass_priority_to(p2.id);
                assert_eq!(
                    answers.priority_holder().map(|player| player.id),
                    Some(p2.id)
                );
                assert!(!answers.have_all_players_passed_in_succession());
                answers.begin_step(turns::Step::Draw);
                assert_eq!(answers.step(), turns::Step::Draw);
                answers.give_priority_to_active_player();
                assert_eq!(
                    answers.priority_holder().map(|player| player.id),
                    Some(p1.id)
                );
                answers.give_priority_to(p2.id);
                assert_eq!(
                    answers.priority_holder().map(|player| player.id),
                    Some(p2.id)
                );
            })
            .unwrap();
    }
}
