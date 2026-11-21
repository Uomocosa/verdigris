use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::{App, Entity};

use crate::cards;
use crate::damage;
use crate::testing;
use crate::turns;
use crate::zones;

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
            let seat = testing::PlayerSeat(index.saturating_add(1));
            let entity = entities.get(index).copied().unwrap_or(Entity::PLACEHOLDER);
            testing::TestPlayer::from_entity_and_seat(entity, seat)
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

impl MockGameBuilder {
    pub(crate) fn spawn_tapped_battlefield(
        app: &mut App,
        player: Entity,
        setup: &testing::PlayerSetup,
        battlefield: Entity,
    ) {
        for definition in setup.tapped_battlefield.iter().cloned() {
            let _ = app.world_mut().spawn((
                definition,
                zones::InZone(battlefield),
                cards::Owner(player),
                damage::Damage(0),
                cards::Tapped,
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{App, Entity, With};

    use crate::cards;
    use crate::testing::*;
    use crate::turns;
    use crate::zones;

    use super::MockGameBuilder;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) =
            MockGame::builder().turn(3, turns::Step::PostcombatMain).players([P1, P2.life(7)]);

        assert_eq!(game.life(p1), 20);
        let turn = game.turn();
        assert_eq!(turn.number, 3);
        assert_eq!(turn.step, turns::Step::PostcombatMain);
    }

    #[test]
    fn tapped_battlefield_spawns_tapped_permanents() {
        let mut app = App::new();
        let player = app.world_mut().spawn_empty().id();
        let battlefield = app.world_mut().spawn_empty().id();
        let setup = P1.battlefield_tapped([GRIZZLY_BEARS]);

        MockGameBuilder::spawn_tapped_battlefield(&mut app, player, &setup, battlefield);

        let mut found = app
            .world_mut()
            .query_filtered::<(Entity, &cards::Owner, &zones::InZone), With<cards::Tapped>>();
        let found: Vec<(Entity, Entity, Entity)> = found
            .iter(app.world())
            .map(|(entity, owner, in_zone)| (entity, **owner, **in_zone))
            .collect();
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].1, player);
        assert_eq!(found[0].2, battlefield);
    }
}
