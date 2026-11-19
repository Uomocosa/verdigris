use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::{App, Entity};

use crate::testing;
use crate::turns;

pub struct MockGameBuilder {
    pub(crate) turn_number: u32,
    pub(crate) step: turns::Step,
}

impl Default for MockGameBuilder {
    fn default() -> Self {
        Self {
            turn_number: 1,
            step: turns::Step::PrecombatMain,
        }
    }
}

impl MockGameBuilder {
    #[must_use]
    pub const fn turn(mut self, number: u32, step: turns::Step) -> Self {
        self.turn_number = number;
        self.step = step;
        self
    }

    #[must_use]
    pub fn players<const N: usize>(
        self,
        players: [testing::PlayerSetup; N],
    ) -> (testing::MockGame, [testing::TestPlayer; N]) {
        let (game, entities) = self.spawn_game(&Vec::from(players));
        let spawned = std::array::from_fn(|index| {
            testing::TestPlayer(entities.get(index).copied().unwrap_or(Entity::PLACEHOLDER))
        });
        (game, spawned)
    }
}

#[atomic_delegates]
impl MockGameBuilder {
    pub(crate) fn spawn_game(
        &self,
        players: &[testing::PlayerSetup],
    ) -> (testing::MockGame, Vec<Entity>) {
    }

    #[must_use]
    pub(crate) fn spawn_players(app: &mut App, players: &[testing::PlayerSetup]) -> Vec<Entity> {}
}

#[cfg(test)]
mod tests {
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::builder()
            .turn(3, turns::Step::PostcombatMain)
            .players([P1, P2.life(7)]);

        assert_eq!(game.life(p1), 20);
        let turn = game.app.world().resource::<turns::Turn>();
        assert_eq!(turn.number, 3);
        assert_eq!(turn.step, turns::Step::PostcombatMain);
    }
}
