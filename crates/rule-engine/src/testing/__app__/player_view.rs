use bevy::prelude::Entity;

use crate::cards;
use crate::testing;
use crate::zones;

pub struct PlayerView<'g> {
    game: &'g testing::MockGame,
    player: testing::TestPlayer,
}

impl<'g> PlayerView<'g> {
    pub(crate) const fn from_game_and_player(
        game: &'g testing::MockGame,
        player: testing::TestPlayer,
    ) -> Self {
        Self { game, player }
    }
}

#[rustfmt::skip]
impl PlayerView<'_> {
    #[must_use]
    pub const fn id(&self) -> Entity { self.player.entity }

    #[must_use]
    pub fn life(&self) -> i32 { self.game.life(self.player) }

    #[must_use]
    pub fn mana_pool(&self) -> cards::ManaPool { self.game.mana_pool(self.player) }
}

impl PlayerView<'_> {
    #[must_use]
    pub fn cards_in(&self, zone: zones::Zone) -> testing::CardsInZone {
        let cards = self.game.cards_in_zone(*self.player, zone);
        testing::CardsInZone::from_cards(cards, zone, self.player)
    }
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::mana;
    use crate::testing::*;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::new([P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]), P2]);
        let view = game.player(p1);
        assert_eq!(view.id(), p1);
        assert_eq!(view.life(), 20);
        assert_eq!(view.mana_pool(), mana![R]);
        assert_eq!(view.cards_in(HAND).len(), 1);
    }
}
