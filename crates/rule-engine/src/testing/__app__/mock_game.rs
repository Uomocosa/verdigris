use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::{App, Entity};

use crate::actions;
use crate::cards;
use crate::testing;
use crate::turns;
use crate::zones;

pub struct MockGame {
    pub(crate) players: Vec<Entity>,
    pub(crate) app: App,
}

impl MockGame {
    #[must_use]
    pub fn new<const N: usize>(players: [testing::PlayerSetup; N]) -> (Self, [Entity; N]) {
        let (game, entities) = Self::builder().players(players);
        (game, entities)
    }
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
}

#[atomic_delegates]
impl MockGame {
    pub fn end_turn(&mut self) {}

    #[must_use]
    pub fn find(&self, player: Entity, zone: zones::Zone, name: &str) -> Option<Entity> {}

    #[must_use]
    pub fn p(&self, seat: usize) -> Entity {}

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
    use crate::testing::*;
    use crate::turns;
    use crate::zones;
    use crate::{BATTLEFIELD, HAND, LIBRARY};

    use super::MockGame;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::new([P1, P2]);
        assert_eq!(game.life(p1), 20);
    }

    #[test]
    fn default_game_has_two_players_twenty_life_and_full_libraries() {
        let (game, [p1, p2]) = MockGame::new([P1, P2]);
        assert_eq!(game.life(p1), 20);
        assert_eq!(game.life(p2), 20);

        let first_bear = game
            .find(p1, LIBRARY, "Grizzly Bears")
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
        assert!(game.find(p1, HAND, "Grizzly Bears").is_none());
        assert!(game.find(p2, HAND, "Grizzly Bears").is_none());

        let turn = game.app.world().resource::<turns::Turn>();
        assert_eq!(turn.number, 1);
        assert_eq!(turn.active, p1);
        assert_eq!(turn.step, turns::Step::PrecombatMain);
        assert_eq!(game.app.world().resource::<turns::Priority>().player, p1);
    }

    #[test]
    fn overrides_place_cards_and_support_four_players() {
        let (game, [p1, p2, ..]) = MockGame::builder()
            .turn(3, turns::Step::PostcombatMain)
            .players([
                P1.hand([lightning_bolt()]).mana_pool(mana![R]),
                P2.active()
                    .battlefield([grizzly_bears()])
                    .library([murder()]),
                P3,
                P4,
            ]);

        assert_ne!(game.p(3), game.p(4));
        assert!(game.app.world().get_entity(game.p(3)).is_ok());
        assert!(game.app.world().get_entity(game.p(4)).is_ok());
        assert!(game.find(p1, HAND, "Lightning Bolt").is_some());
        assert!(game.find(p2, BATTLEFIELD, "Grizzly Bears").is_some());
        assert!(game.find(p2, LIBRARY, "Murder").is_some());
        assert!(game.find(p2, LIBRARY, "Grizzly Bears").is_none());
        assert_eq!(game.mana_pool(p1), mana![R]);

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
    fn new_builds_the_described_players() {
        let (game, [p1, p2]) = MockGame::new([P1.hand([divination()]).mana_pool(mana![2, U]), P2]);

        assert!(game.find(p1, HAND, "Divination").is_some());
        assert!(game.find(p1, HAND, "Grizzly Bears").is_none());
        assert_eq!(game.life(p1), 20);
        assert_eq!(game.life(p2), 20);
        assert_eq!(game.mana_pool(p1), mana![2, U]);
        assert_eq!(game.mana_pool(p2), mana![]);

        let turn = game.app.world().resource::<turns::Turn>();
        assert_eq!(turn.number, 1);
        assert_eq!(turn.step, turns::Step::PrecombatMain);
        assert_eq!(turn.active, p1);
        assert_eq!(game.app.world().resource::<turns::Priority>().player, p1);
    }

    #[test]
    fn builder_players_marks_the_active_setup_and_its_turn() {
        let (game, [_, p2, ..]) = MockGame::builder()
            .turn(3, turns::Step::PostcombatMain)
            .players([P1, P2.active().battlefield([grizzly_bears()]), P3, P4]);

        assert!(game.find(p2, BATTLEFIELD, "Grizzly Bears").is_some());

        let turn = game.app.world().resource::<turns::Turn>();
        assert_eq!(turn.number, 3);
        assert_eq!(turn.step, turns::Step::PostcombatMain);
        assert_eq!(turn.active, p2);
        assert_eq!(game.app.world().resource::<turns::Priority>().player, p2);
    }

    #[test]
    #[should_panic(expected = "MockGame: P2 listed in position 1")]
    fn players_rejects_a_setup_out_of_seat_order() {
        let _ = MockGame::new([P2, P1]);
    }

    #[test]
    #[should_panic(expected = "MockGame: P1 and P2 are both marked .active()")]
    fn players_rejects_two_active_setups() {
        let _ = MockGame::new([P1.active(), P2.active()]);
    }

    #[test]
    fn fresh_game_has_no_target_or_rejection() {
        let (game, [p1, ..]) = MockGame::new([P1, P2]);
        assert!(game.find(p1, HAND, "Lightning Bolt").is_none());
        assert!(game.target_choice().is_none());
        assert!(game.last_rejection().is_none());
    }

    #[test]
    fn act_and_pass_priority_advance_the_app() {
        let (mut game, [p1, ..]) = MockGame::new([P1, P2]);
        game.act(p1, actions::Action::PassPriority);
        game.pass_priority_all();
        assert!(game.last_rejection().is_none());
    }
}
