use atomic_delegate_macros::atomic_delegates;

use crate::testing;
use crate::zones;

#[derive(Clone, Copy)]
pub struct CardCheck<'g> {
    pub(crate) game: &'g testing::MockGame,
    pub(crate) card: testing::TestCard,
}

impl<'g> CardCheck<'g> {
    pub(crate) const fn from_game_and_card(
        game: &'g testing::MockGame,
        card: testing::TestCard,
    ) -> Self {
        Self { game, card }
    }
}

#[atomic_delegates]
impl CardCheck<'_> {
    #[must_use]
    pub fn is_on_the_stack(self) -> bool {}
}

#[rustfmt::skip]
impl<'g> CardCheck<'g> {
    #[must_use]
    pub const fn is_in(self, zone: zones::Zone) -> testing::ZoneCheck<'g> { testing::ZoneCheck::from_game_card_and_zone(self.game, self.card, zone) }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::HAND;
    use crate::testing::*;

    #[test]
    fn test_usage() -> TestResult {
        let (game, [p1, p2]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;

        assert!(game.card(bears).is_in(BATTLEFIELD).of(p1));
        assert!(!game.card(bears).is_in(BATTLEFIELD).of(p2));
        assert!(!game.card(bears).is_in(HAND).of(p1));
        Ok(())
    }
}
