use crate::testing;
use crate::zones;

pub struct MoveCheck<'g> {
    game: &'g testing::MockGame,
    card: testing::TestCard,
}

impl<'g> MoveCheck<'g> {
    pub(crate) const fn from_game_and_card(
        game: &'g testing::MockGame,
        card: testing::TestCard,
    ) -> Self {
        Self { game, card }
    }
}

#[rustfmt::skip]
impl<'g> MoveCheck<'g> {
    #[must_use]
    pub const fn to(self, zone: zones::Zone) -> testing::MoveDestination<'g> { testing::MoveDestination::from_game_card_and_zone(self.game, self.card, zone) }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::testing::*;

    #[test]
    fn test_usage() -> TestResult {
        let (game, [p1, ..]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;

        assert!(!game.moved(bears).to(BATTLEFIELD).of(p1));
        Ok(())
    }
}
