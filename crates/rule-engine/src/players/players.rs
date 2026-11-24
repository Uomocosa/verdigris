use bevy::ecs::system::SystemParam;
use bevy::prelude::{Entity, Query, With};

use crate::players;
use crate::turns;

#[derive(SystemParam, Clone, Copy)]
pub struct Players<'w, 's> {
    turn_order: Query<'w, 's, &'static turns::TurnOrder>,
    still_in_the_game: Query<'w, 's, Entity, players::StillInTheGame>,
    winners: Query<'w, 's, Entity, With<players::Won>>,
    drawn: Query<'w, 's, Entity, With<players::Drawn>>,
}

#[rustfmt::skip]
impl Players<'_, '_> {
    #[must_use]
    pub fn in_turn_order(&self) -> Vec<Entity> { self.turn_order.iter().flat_map(|order| order.iter().copied()).collect() }

    #[must_use]
    pub fn opponents_of(&self, player: Entity) -> Vec<Entity> { self.in_turn_order().into_iter().filter(|other| *other != player).collect() }

    #[must_use]
    pub fn is_still_in_the_game(&self, player: Entity) -> bool { self.still_in_the_game.contains(player) }

    #[must_use]
    pub fn is_game_over(&self) -> bool { !self.winners.is_empty() || !self.drawn.is_empty() }
}

impl Players<'_, '_> {
    #[must_use]
    pub fn still_in_the_game(&self) -> Vec<Entity> {
        let in_order = self.in_turn_order();
        in_order.into_iter().filter(|player| self.is_still_in_the_game(*player)).collect()
    }

    #[must_use]
    pub fn have_all_opponents_left(&self, player: Entity) -> bool {
        let opponents = self.opponents_of(player);
        let is_any_still_here = opponents.iter().any(|other| self.is_still_in_the_game(*other));
        !opponents.is_empty() && !is_any_still_here
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::Entity;

    use crate::testing::*;

    use super::Players;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2, p3]) = MockGame::new([P1, P2.life(0), P3]);
        game.play([p1.pass_priority()])?;
        game.app
            .world_mut()
            .run_system_once(move |players: Players| {
                assert_eq!(
                    players.still_in_the_game(),
                    [Entity::from(p1), Entity::from(p3)]
                );
                assert_eq!(
                    players.opponents_of(Entity::from(p1)),
                    [Entity::from(p2), Entity::from(p3)]
                );
                assert!(!players.is_still_in_the_game(Entity::from(p2)));
                assert!(!players.have_all_opponents_left(Entity::from(p1)));
                assert!(!players.is_game_over());
            })
            .unwrap();
        Ok(())
    }
}
