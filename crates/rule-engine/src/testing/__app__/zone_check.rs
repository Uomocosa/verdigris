use crate::testing;
use crate::zones;

pub struct ZoneCheck<'g> {
    game: &'g testing::MockGame,
    card: testing::TestCard,
    zone: zones::Zone,
}

impl<'g> ZoneCheck<'g> {
    pub(crate) const fn from_game_card_and_zone(
        game: &'g testing::MockGame,
        card: testing::TestCard,
        zone: zones::Zone,
    ) -> Self {
        Self { game, card, zone }
    }
}

impl ZoneCheck<'_> {
    #[must_use]
    pub fn of(self, player: testing::TestPlayer) -> bool {
        let query = testing::CardQuery::Exact(self.card);
        let objects = self.game.cards_in_zone(player.id, self.zone);
        objects.iter().any(|object| query.matches(object).is_some())
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::GRAVEYARD;
    use crate::testing::*;

    #[test]
    fn test_usage() -> TestResult {
        let (game, [p1, p2]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;

        assert!(game.card(bears).is_in(BATTLEFIELD).of(p1));
        assert!(!game.card(bears).is_in(BATTLEFIELD).of(p2));
        assert!(!game.card(bears).is_in(GRAVEYARD).of(p1));
        Ok(())
    }
}
