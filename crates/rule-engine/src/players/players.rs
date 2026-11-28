use atomic_delegate_macros::atomic_delegates;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Entity, Query, With};

use crate::cards;
use crate::players;
use crate::rules;
use crate::turns;
use crate::zones;

#[derive(SystemParam, Clone, Copy)]
pub struct Players<'w, 's> {
    pub(crate) turn_order: Query<'w, 's, &'static turns::TurnOrder>,
    statuses: Query<'w, 's, players::Status, With<players::Player>>,
    pub(crate) zone_contents: Query<
        'w,
        's,
        (
            &'static zones::Zone,
            &'static cards::Owner,
            &'static zones::ZoneContents,
        ),
    >,
}

#[rustfmt::skip]
impl Players<'_, '_> {
    #[must_use]
    pub fn is_still_in_the_game(&self, player: players::PlayerId) -> bool { self.statuses.get(*player).is_ok_and(|status| !status.has_left_the_game) }

    #[must_use]
    pub fn has_lost(&self, player: players::PlayerId) -> bool { self.statuses.get(*player).is_ok_and(|status| status.has_lost) }

    #[must_use]
    pub fn has_anyone_won(&self) -> bool { self.statuses.iter().any(|status| status.has_won) }

    #[must_use]
    pub fn is_two_player_game(&self) -> bool { self.in_turn_order().len() == 2 }

    #[must_use]
    pub fn in_turn_order(&self) -> Vec<players::PlayerId> { self.turn_order.iter().flat_map(|order| order.iter().copied()).collect() }

    #[must_use]
    pub fn as_player(&self, entity: Entity) -> Option<players::PlayerId> { self.statuses.contains(entity).then_some(players::PlayerId(entity)) }
}

#[atomic_delegates]
impl Players<'_, '_> {
    #[must_use]
    pub fn still_in_the_game(&self) -> Vec<players::PlayerView<'_, '_>> {}

    #[must_use]
    pub fn players_who_left(&self) -> Vec<players::PlayerId> {}

    #[must_use]
    pub fn affected_by(
        &self,
        event: &impl rules::traits::event::Event,
    ) -> Vec<players::PlayerView<'_, '_>> {
    }
}

#[rustfmt::skip]
impl<'w, 's> Players<'w, 's> {
    #[must_use]
    pub(crate) const fn view(self, id: players::PlayerId) -> players::PlayerView<'w, 's> { players::PlayerView { id, players: self } }

    #[must_use]
    pub(crate) fn is_a_draw_for_anyone(&self) -> bool { self.statuses.iter().any(|status| status.is_game_a_draw) }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::Entity;

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
                assert!(!players.is_two_player_game());
                assert_eq!(players.as_player(*p1), Some(p1.id));
                assert_eq!(players.as_player(Entity::PLACEHOLDER), None);
            })
            .unwrap();
        Ok(())
    }
}
