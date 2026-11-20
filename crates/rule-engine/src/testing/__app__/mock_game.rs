use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::{App, Entity};

use crate::actions;
use crate::cards;
use crate::players;
use crate::testing;
use crate::turns;
use crate::zones;

pub struct MockGame {
    pub(crate) players: Vec<Entity>,
    pub(crate) app: App,
}

impl MockGame {
    #[must_use]
    pub fn new<const N: usize>(
        players: [testing::PlayerSetup; N],
    ) -> (Self, [testing::TestPlayer; N]) {
        let (game, entities) = Self::builder().players(players);
        (game, entities)
    }
}

#[rustfmt::skip]
impl MockGame {
    #[must_use]
    pub fn builder() -> testing::MockGameBuilder { testing::MockGameBuilder::default() }

    #[must_use]
    pub const fn player(&self, player: testing::TestPlayer) -> testing::PlayerView<'_> { testing::PlayerView::from_game_and_player(self, player) }

    #[must_use]
    pub fn life(&self, player: impl Into<Entity>) -> i32 { self.app.world().get::<players::Life>(player.into()).map_or(0, |life| **life) }

    #[must_use]
    pub fn mana_pool(&self, player: impl Into<Entity>) -> cards::ManaPool { self.app.world().get::<cards::ManaPool>(player.into()).copied().unwrap_or_default() }
}

impl MockGame {
    pub fn act(&mut self, player: impl Into<Entity>, action: actions::Action) {
        let player = player.into();
        let _ = self.app.world_mut().write_message(actions::PlayerAction { player, action });
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
        let passes = self.app.world().resource::<turns::Priority>().consecutive_passes;
        let player_count = u32::try_from(self.players.len()).unwrap_or(u32::MAX);
        for _ in passes..player_count {
            self.pass_priority();
        }
    }
}

#[atomic_delegates]
impl MockGame {
    #[must_use]
    pub fn p(&self, seat: usize) -> testing::TestPlayer {}

    #[must_use]
    pub fn zone_of(&self, card: impl Into<Entity>) -> zones::Zone {}

    #[must_use]
    pub fn target_choice(&self) -> Option<testing::ExpectedTargets> {}

    #[must_use]
    pub fn last_rejection(&self) -> Option<&actions::Rejected> {}

    pub fn play(
        &mut self,
        actions: impl IntoIterator<Item = testing::TestPlayerActions>,
    ) -> Result<(), testing::PlayError> {
    }
}

#[atomic_delegates]
impl MockGame {
    pub(crate) fn cards_in_zone(
        &self,
        player: Entity,
        zone: zones::Zone,
    ) -> Vec<(testing::TestCard, cards::CardName)> {
    }

    pub(crate) fn rejection_count(&self) -> usize {}
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::actions;
    use crate::cards;
    use crate::mana;
    use crate::testing::*;
    use crate::turns;
    use crate::{BATTLEFIELD, HAND, LIBRARY};

    use super::MockGame;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::new([P1, P2]);
        assert_eq!(game.life(p1), 20);
    }

    #[test]
    fn default_game_has_two_players_twenty_life_and_full_libraries() -> Result<(), TestError> {
        let (game, [p1, p2]) = MockGame::new([P1, P2]);
        assert_eq!(game.life(p1), 20);
        assert_eq!(game.life(p2), 20);

        for player in [p1, p2] {
            let library = game.player(player).cards_in(LIBRARY);
            assert_eq!(library.len(), 12);
            library.exactly::<12>(GRIZZLY_BEARS)?;
        }
        assert!(game.player(p1).cards_in(HAND).has_no(GRIZZLY_BEARS));
        assert!(game.player(p2).cards_in(HAND).has_no(GRIZZLY_BEARS));

        let turn = game.app.world().resource::<turns::Turn>();
        assert_eq!(turn.number, 1);
        assert_eq!(turn.active, p1);
        assert_eq!(turn.step, turns::Step::PrecombatMain);
        assert_eq!(game.app.world().resource::<turns::Priority>().player, p1);
        Ok(())
    }

    #[test]
    fn overrides_place_cards_and_support_four_players() {
        let (game, [p1, p2, ..]) =
            MockGame::builder().turn(3, turns::Step::PostcombatMain).players([
                P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]),
                P2.active()
                    .battlefield([GRIZZLY_BEARS])
                    .library([cards::CardDefinition::from(MURDER)]),
                P3,
                P4,
            ]);

        assert_ne!(game.p(3), game.p(4));
        assert!(game.app.world().get_entity(Entity::from(game.p(3))).is_ok());
        assert!(game.app.world().get_entity(Entity::from(game.p(4))).is_ok());
        assert!(game.player(p1).cards_in(HAND).has(LIGHTNING_BOLT));
        assert!(game.player(p2).cards_in(BATTLEFIELD).has(GRIZZLY_BEARS));
        assert!(game.player(p2).cards_in(LIBRARY).has(MURDER));
        assert!(game.player(p2).cards_in(LIBRARY).has_no(GRIZZLY_BEARS));
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
        let (game, [p1, p2]) = MockGame::new([P1.hand([DIVINATION]).mana_pool(mana![2, U]), P2]);

        assert!(game.player(p1).cards_in(HAND).has(DIVINATION));
        assert!(game.player(p1).cards_in(HAND).has_no(GRIZZLY_BEARS));
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
            .players([P1, P2.active().battlefield([GRIZZLY_BEARS]), P3, P4]);

        assert!(game.player(p2).cards_in(BATTLEFIELD).has(GRIZZLY_BEARS));

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
        assert!(game.player(p1).cards_in(HAND).has_no(LIGHTNING_BOLT));
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
