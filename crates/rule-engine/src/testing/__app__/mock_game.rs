use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::{App, Entity};

use crate::actions;
use crate::cards;
use crate::testing;
use crate::turns;
use crate::zones;

pub struct MockGame {
    pub p1: Entity,
    pub p2: Entity,
    pub(crate) players: Vec<Entity>,
    pub(crate) app: App,
}

#[rustfmt::skip]
impl MockGame {
    #[must_use]
    pub fn builder() -> testing::MockGameBuilder { testing::MockGameBuilder::default() }
}

impl MockGame {
    pub fn act(&mut self, player: Entity, action: actions::Action) {
        let _ = self
            .app
            .world_mut()
            .write_message(actions::PlayerAction { player, action });
        self.app.update();
    }

    pub fn pass_priority(&mut self) {
        let player = self.app.world().resource::<turns::Priority>().player;
        let _ = self.app.world_mut().write_message(actions::PlayerAction {
            player,
            action: actions::Action::PassPriority,
        });
        self.app.update();
    }

    pub fn pass_priority_all(&mut self) {
        let passes = self
            .app
            .world()
            .resource::<turns::Priority>()
            .consecutive_passes;
        let player_count = u32::try_from(self.players.len()).unwrap_or(u32::MAX);
        for _ in passes..player_count {
            self.pass_priority();
        }
    }

    #[must_use]
    pub fn p(&self, seat: usize) -> Entity {
        let index = seat.saturating_sub(1);
        self.players
            .get(index)
            .copied()
            .unwrap_or(Entity::PLACEHOLDER)
    }
}

#[atomic_delegates]
impl MockGame {
    pub fn cleanup(&mut self) {}

    #[must_use]
    pub fn find(&self, player: Entity, zone: zones::Zone, name: &str) -> Option<Entity> {}

    #[must_use]
    pub fn zone_of(&self, card: Entity) -> zones::Zone {}

    #[must_use]
    pub fn target_choice(&self) -> Option<testing::ExpectedTargets> {}

    #[must_use]
    pub fn last_rejection(&self) -> Option<&actions::Rejected> {}

    #[must_use]
    pub fn life(&self, player: Entity) -> i32 {}

    #[must_use]
    pub fn mana_pool(&self, player: Entity) -> cards::ManaPool {}
}

#[cfg(test)]
mod tests {
    use crate::actions;
    use crate::mana;
    use crate::testing;
    use crate::turns;
    use crate::zones;

    use super::MockGame;

    #[test]
    fn test_usage() {
        let game = MockGame::builder().build();
        assert_eq!(game.life(game.p1), 20);
    }

    #[test]
    fn default_game_has_two_players_twenty_life_and_full_libraries() {
        let game = MockGame::builder().build();
        assert_eq!(game.life(game.p1), 20);
        assert_eq!(game.life(game.p2), 20);

        let first_bear = game
            .find(game.p1, zones::Zone::Library, "Grizzly Bears")
            .expect("P1 library holds Grizzly Bears");
        let library = *game.app.world().get::<zones::InZone>(first_bear).unwrap();
        let bears_in_library = game
            .app
            .world()
            .iter_entities()
            .filter(|entity| {
                entity
                    .get::<zones::InZone>()
                    .is_some_and(|in_zone| *in_zone == library)
            })
            .count();
        assert_eq!(bears_in_library, 12);
        assert!(
            game.find(game.p1, zones::Zone::Hand, "Grizzly Bears")
                .is_none()
        );
        assert!(
            game.find(game.p2, zones::Zone::Hand, "Grizzly Bears")
                .is_none()
        );

        let turn = game.app.world().resource::<turns::Turn>();
        assert_eq!(turn.number, 1);
        assert_eq!(turn.active, game.p1);
        assert_eq!(turn.step, turns::Step::PrecombatMain);
        assert_eq!(
            game.app.world().resource::<turns::Priority>().player,
            game.p1
        );
    }

    #[test]
    fn overrides_place_cards_and_support_four_players() {
        let game = MockGame::builder()
            .players(4)
            .turn(3, testing::Seat::P2, turns::Step::PostcombatMain)
            .hand(testing::Seat::P1, [testing::lightning_bolt()])
            .battlefield(testing::Seat::P2, [testing::grizzly_bears()])
            .library(testing::Seat::P2, [testing::murder()])
            .mana_pool(testing::Seat::P1, mana![R])
            .build();

        assert_ne!(game.p(3), game.p(4));
        assert!(game.app.world().get_entity(game.p(3)).is_ok());
        assert!(game.app.world().get_entity(game.p(4)).is_ok());
        assert!(
            game.find(game.p1, zones::Zone::Hand, "Lightning Bolt")
                .is_some()
        );
        assert!(
            game.find(game.p2, zones::Zone::Battlefield, "Grizzly Bears")
                .is_some()
        );
        assert!(game.find(game.p2, zones::Zone::Library, "Murder").is_some());
        assert!(
            game.find(game.p2, zones::Zone::Library, "Grizzly Bears")
                .is_none()
        );
        assert_eq!(game.mana_pool(game.p1), mana![R]);

        let turn = game.app.world().resource::<turns::Turn>();
        assert_eq!(turn.number, 3);
        assert_eq!(turn.active, game.p(2));
        assert_eq!(turn.step, turns::Step::PostcombatMain);
        assert_eq!(
            game.app.world().resource::<turns::Priority>().player,
            game.p(2)
        );
    }

    #[test]
    fn fresh_game_has_no_target_or_rejection() {
        let game = MockGame::builder().build();
        assert!(
            game.find(game.p1, zones::Zone::Hand, "Lightning Bolt")
                .is_none()
        );
        assert!(game.target_choice().is_none());
        assert!(game.last_rejection().is_none());
    }

    #[test]
    fn act_and_pass_priority_advance_the_app() {
        let mut game = MockGame::builder().build();
        game.act(game.p1, actions::Action::PassPriority);
        game.pass_priority_all();
        assert!(game.last_rejection().is_none());
    }
}
