use std::collections::BTreeMap;

use atomic_delegate_macros::atomic_delegates;
use bevy::ecs::query::QueryData;
use bevy::prelude::{App, Component, Entity, Query};

use crate::actions;
use crate::cards;
use crate::damage;
use crate::players;
use crate::testing;
use crate::turns;
use crate::zones;

pub struct MockGame {
    pub(crate) players: Vec<players::PlayerId>,
    pub(crate) zones: Vec<BTreeMap<zones::Zone, Entity>>,
    pub(crate) stack: Entity,
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

    #[must_use]
    pub fn has<C: Component>(&self, entity: impl Into<Entity>) -> bool { self.app.world().get::<C>(entity.into()).is_some() }

    #[must_use]
    pub fn get<C: Component + Copy>(&self, entity: impl Into<Entity>) -> Option<C> { self.app.world().get::<C>(entity.into()).copied() }

    #[must_use]
    pub fn is_attacking(&self, card: impl Into<Entity>) -> bool { self.app.world().get::<cards::Attacking>(card.into()).is_some() }

    #[must_use]
    pub fn is_blocking(&self, card: impl Into<Entity>) -> bool { self.app.world().get::<cards::Blocking>(card.into()).is_some() }

    #[must_use]
    pub fn is_tapped(&self, card: impl Into<Entity>) -> bool { self.app.world().get::<cards::Tapped>(card.into()).is_some() }

    #[must_use]
    pub fn has_won(&self, player: impl Into<Entity>) -> bool { self.app.world().get::<players::Won>(player.into()).is_some() }

    #[must_use]
    pub fn has_lost(&self, player: impl Into<Entity>) -> bool { self.app.world().get::<players::Lost>(player.into()).is_some() }

    #[must_use]
    pub fn damage(&self, card: impl Into<Entity>) -> u32 { self.app.world().get::<damage::Damage>(card.into()).map_or(0, |damage| **damage) }

    #[must_use]
    pub fn pt(&self, card: impl Into<Entity>) -> Option<cards::PowerToughness> { self.app.world().get::<cards::EffectivePT>(card.into()).map(|effective| **effective) }

    #[must_use]
    pub fn turn(&self) -> &turns::Turn { &self.app.world().resource::<turns::TurnState>().turn }

    #[must_use]
    pub fn priority(&self) -> turns::Priority {
        self.app.world().resource::<turns::TurnState>().priority.clone()
    }

    #[must_use]
    pub fn priority_player(&self) -> Option<players::PlayerId> { self.priority().holder }
}

impl MockGame {
    #[must_use]
    pub fn zone_entity(
        &self,
        player: testing::TestPlayer,
        zone: impl Into<zones::Zone>,
    ) -> Option<Entity> {
        let zone = zone.into();
        if zone == zones::Zone::Stack {
            Some(self.stack)
        } else {
            self.zones.get((*player.seat).saturating_sub(1))?.get(&zone).copied()
        }
    }

    pub fn act(&mut self, player: impl Into<Entity>, action: actions::Action) {
        let player = players::PlayerId(player.into());
        let _ = self.app.world_mut().write_message(actions::PlayerAction { player, action });
        self.app.update();
    }

    pub fn pass_priority(&mut self) {
        let player = self.priority().holder.unwrap_or_else(|| self.turn().active);
        let _ = self.app.world_mut().write_message(actions::PlayerAction {
            player,
            action: actions::Action::PassPriority,
        });
        self.app.update();
    }

    pub fn pass_priority_all(&mut self) {
        let remaining = self
            .players
            .iter()
            .filter(|player| {
                let id: players::PlayerId = **player;
                !self.priority().has_passed(id) && !self.has_lost(*id)
            })
            .count();
        for _ in 0..remaining {
            self.pass_priority();
        }
    }
}

#[atomic_delegates]
impl MockGame {
    #[must_use]
    pub fn p(&self, seat: usize) -> testing::TestPlayer {}

    #[must_use]
    pub fn stack(&self) -> testing::CardsInZone {}

    #[must_use]
    pub fn target_choice(&self) -> Option<testing::ExpectedTargets> {}

    #[must_use]
    pub fn trigger_sources(&self) -> Vec<Entity> {}

    #[must_use]
    pub fn last_rejection(&self) -> Option<&actions::Rejected> {}

    pub fn play(
        &mut self,
        actions: impl IntoIterator<Item = testing::TestPlayerActions>,
    ) -> Result<(), testing::PlayError> {
    }

    #[must_use]
    pub fn with_query<D: QueryData, R>(
        &mut self,
        with: impl FnOnce(Query<'_, '_, D::ReadOnly>) -> R,
    ) -> R {
    }

    #[must_use]
    pub fn with_queries<D: QueryData, E: QueryData, R>(
        &mut self,
        with: impl FnOnce(Query<'_, '_, D::ReadOnly>, Query<'_, '_, E::ReadOnly>) -> R,
    ) -> R {
    }
}

#[atomic_delegates]
impl MockGame {
    pub(crate) fn cards_in_zone(
        &self,
        player: players::PlayerId,
        zone: zones::Zone,
    ) -> Vec<testing::ZoneObject> {
    }

    pub(crate) fn stack_objects(&self) -> Vec<testing::ZoneObject> {}

    pub(crate) fn rejection_count(&self) -> usize {}
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::actions;
    use crate::mana;
    use crate::players;
    use crate::testing::*;
    use crate::turns;
    use crate::{BATTLEFIELD, HAND, LIBRARY};

    use super::MockGame;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1, P2.battlefield([GRIZZLY_BEARS])]);
        assert_eq!(game.life(p1), 20);
        assert!(game.has::<players::Life>(p1));
        assert_eq!(game.get::<players::Life>(p1), Some(players::Life(20)));
        assert!(!game.is_tapped(p1));
        assert!(!game.has_won(p1));
        assert!(!game.has_lost(p1));
        assert_eq!(game.damage(p1), 0);
        assert_eq!(game.turn().number, 1);
        assert_eq!(game.priority_player(), None);
        assert_eq!(game.priority().holder, None);
        assert!(game.priority().passed.is_empty());
        assert_eq!(game.player(p2).cards_in(BATTLEFIELD).count(), 1);
        assert_eq!(game.stack().count(), 0);

        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        assert!(!game.is_attacking(bears));
        assert!(!game.is_blocking(bears));
        assert!(!game.is_tapped(bears));
        assert_eq!(game.damage(bears), 0);
        game.app.update();
        assert_eq!(game.pt(bears), Some(crate::pt!(2, 2)));
        Ok(())
    }

    #[test]
    fn default_game_has_two_players_twenty_life_and_full_libraries() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1, P2]);
        game.app.update();
        assert_eq!(game.life(p1), 20);
        assert_eq!(game.life(p2), 20);

        for player in [p1, p2] {
            let library = game.player(player).cards_in(LIBRARY);
            assert_eq!(library.count(), 12);
            library.exactly::<12>(GRIZZLY_BEARS)?;
        }
        assert!(game.player(p1).cards_in(HAND).has_no(GRIZZLY_BEARS));
        assert!(game.player(p2).cards_in(HAND).has_no(GRIZZLY_BEARS));

        let turn = game.turn();
        assert_eq!(turn.number, 1);
        assert_eq!(turn.active, p1.id);
        assert_eq!(turn.step, turns::Step::PrecombatMain);
        assert_eq!(game.priority_player(), Some(p1.id));
        Ok(())
    }

    #[test]
    fn overrides_place_cards_and_support_four_players() {
        let (mut game, [p1, p2, ..]) =
            MockGame::builder().turn(3, turns::Step::PostcombatMain).players([
                P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]),
                P2.active().battlefield([GRIZZLY_BEARS]).library([MURDER]),
                P3,
                P4,
            ]);
        game.app.update();

        assert_ne!(game.p(3), game.p(4));
        assert!(game.app.world().get_entity(Entity::from(game.p(3))).is_ok());
        assert!(game.app.world().get_entity(Entity::from(game.p(4))).is_ok());
        assert!(game.player(p1).cards_in(HAND).has(LIGHTNING_BOLT));
        assert!(game.player(p2).cards_in(BATTLEFIELD).has(GRIZZLY_BEARS));
        assert!(game.player(p2).cards_in(LIBRARY).has(MURDER));
        assert!(game.player(p2).cards_in(LIBRARY).has_no(GRIZZLY_BEARS));
        assert_eq!(game.mana_pool(p1), mana![R]);

        let turn = game.turn();
        assert_eq!(turn.number, 3);
        assert_eq!(turn.active, game.p(2).id);
        assert_eq!(turn.step, turns::Step::PostcombatMain);
        assert_eq!(game.priority_player(), Some(game.p(2).id));
    }

    #[test]
    fn new_builds_the_described_players() {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([DIVINATION]).mana_pool(mana![2, U]), P2]);
        game.app.update();

        assert!(game.player(p1).cards_in(HAND).has(DIVINATION));
        assert!(game.player(p1).cards_in(HAND).has_no(GRIZZLY_BEARS));
        assert_eq!(game.life(p1), 20);
        assert_eq!(game.life(p2), 20);
        assert_eq!(game.mana_pool(p1), mana![2, U]);
        assert_eq!(game.mana_pool(p2), mana![]);

        let turn = game.turn();
        assert_eq!(turn.number, 1);
        assert_eq!(turn.step, turns::Step::PrecombatMain);
        assert_eq!(turn.active, p1.id);
        assert_eq!(game.priority_player(), Some(p1.id));
    }

    #[test]
    fn builder_players_marks_the_active_setup_and_its_turn() {
        let (mut game, [_, p2, ..]) = MockGame::builder()
            .turn(3, turns::Step::PostcombatMain)
            .players([P1, P2.active().battlefield([GRIZZLY_BEARS]), P3, P4]);
        game.app.update();

        assert!(game.player(p2).cards_in(BATTLEFIELD).has(GRIZZLY_BEARS));

        let turn = game.turn();
        assert_eq!(turn.number, 3);
        assert_eq!(turn.step, turns::Step::PostcombatMain);
        assert_eq!(turn.active, p2.id);
        assert_eq!(game.priority_player(), Some(p2.id));
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
