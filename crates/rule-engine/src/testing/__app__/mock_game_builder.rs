use std::collections::BTreeMap;

use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::{App, Entity};

use crate::cards;
use crate::testing;
use crate::turns;

pub struct MockGameBuilder {
    pub(crate) player_count: usize,
    pub(crate) turn_number: u32,
    pub(crate) active: testing::Seat,
    pub(crate) step: turns::Step,
    pub(crate) hands: BTreeMap<testing::Seat, Vec<cards::CardDefinition>>,
    pub(crate) battlefields: BTreeMap<testing::Seat, Vec<cards::CardDefinition>>,
    pub(crate) libraries: BTreeMap<testing::Seat, Vec<cards::CardDefinition>>,
    pub(crate) mana_pools: BTreeMap<testing::Seat, cards::ManaPool>,
    pub(crate) lives: BTreeMap<testing::Seat, i32>,
}

impl Default for MockGameBuilder {
    fn default() -> Self {
        Self {
            player_count: 2,
            turn_number: 1,
            active: testing::Seat::P1,
            step: turns::Step::PrecombatMain,
            hands: BTreeMap::new(),
            battlefields: BTreeMap::new(),
            libraries: BTreeMap::new(),
            mana_pools: BTreeMap::new(),
            lives: BTreeMap::new(),
        }
    }
}

impl MockGameBuilder {
    #[must_use]
    pub const fn player_count(mut self, count: usize) -> Self {
        self.player_count = count;
        self
    }

    #[must_use]
    pub fn hand(
        mut self,
        seat: testing::Seat,
        definitions: impl IntoIterator<Item = cards::CardDefinition>,
    ) -> Self {
        self.hands.entry(seat).or_default().extend(definitions);
        self
    }

    #[must_use]
    pub fn battlefield(
        mut self,
        seat: testing::Seat,
        definitions: impl IntoIterator<Item = cards::CardDefinition>,
    ) -> Self {
        self.battlefields
            .entry(seat)
            .or_default()
            .extend(definitions);
        self
    }

    #[must_use]
    pub fn library(
        mut self,
        seat: testing::Seat,
        definitions: impl IntoIterator<Item = cards::CardDefinition>,
    ) -> Self {
        self.libraries
            .insert(seat, definitions.into_iter().collect());
        self
    }

    #[must_use]
    pub fn mana_pool(mut self, seat: testing::Seat, pool: cards::ManaPool) -> Self {
        self.mana_pools.insert(seat, pool);
        self
    }

    #[must_use]
    pub fn life(mut self, seat: testing::Seat, amount: i32) -> Self {
        self.lives.insert(seat, amount);
        self
    }

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
    ) -> (testing::MockGame, [Entity; N]) {
        let (game, entities) = self.spawn_game(&Vec::from(players));
        let spawned = std::array::from_fn(|index| {
            entities.get(index).copied().unwrap_or(Entity::PLACEHOLDER)
        });
        (game, spawned)
    }
}

#[atomic_delegates]
impl MockGameBuilder {
    #[must_use]
    pub fn turn_seat(self, number: u32, active: testing::Seat, step: turns::Step) -> Self {}

    #[must_use]
    pub fn build(&self) -> testing::MockGame {}
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

    #[test]
    fn test_usage() {
        let game = MockGame::builder()
            .player_count(3)
            .life(Seat::P2, 7)
            .build();
        assert_eq!(game.life(game.p1), 20);
        assert_eq!(game.life(game.p2), 7);
        assert_eq!(game.life(game.p(3)), 20);
    }
}
