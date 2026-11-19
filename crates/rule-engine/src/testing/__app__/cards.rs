use atomic_delegate_macros::atomic_delegates;

use crate::testing;
use crate::zones;

pub struct Cards {
    matches: Vec<testing::TestCard>,
    card: String,
    zone: zones::Zone,
    player: testing::TestPlayer,
}

impl Cards {
    pub(crate) const fn from_matches(
        cards: Vec<testing::TestCard>,
        card: String,
        zone: zones::Zone,
        player: testing::TestPlayer,
    ) -> Self {
        Self {
            matches: cards,
            card,
            zone,
            player,
        }
    }
}

#[rustfmt::skip]
impl Cards {
    #[must_use]
    pub fn all(&self) -> &[testing::TestCard] { &self.matches }

    #[must_use]
    pub const fn is_empty(&self) -> bool { self.matches.is_empty() }

    #[must_use]
    pub const fn len(&self) -> usize { self.matches.len() }
}

#[atomic_delegates]
impl Cards {
    pub fn expect_n<const N: usize>(&self) -> Result<[testing::TestCard; N], testing::CardsError> {}

    pub fn first(&self) -> Result<testing::TestCard, testing::CardsError> {}

    pub fn nth(&self, index: usize) -> Result<testing::TestCard, testing::CardsError> {}
}

impl Cards {
    pub fn only(&self) -> Result<testing::TestCard, testing::CardsError> {
        match self.matches.as_slice() {
            [card] => Ok(*card),
            [] => Err(testing::CardsError::Missing {
                player: self.player.seat,
                card: self.card.clone(),
                zone: self.zone,
            }),
            _ => Err(testing::CardsError::NotOne {
                player: self.player.seat,
                card: self.card.clone(),
                zone: self.zone,
                count: self.matches.len(),
            }),
        }
    }
}

#[rustfmt::skip]
impl Cards {
    pub(crate) fn card(&self) -> &str { &self.card }

    pub(crate) const fn seat(&self) -> testing::PlayerSeat { self.player.seat }

    pub(crate) const fn zone(&self) -> zones::Zone { self.zone }
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::testing::*;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::new([P1.hand([LIGHTNING_BOLT]), P2]);
        let cards = game.player(p1).cards_in(HAND).matching(LIGHTNING_BOLT);
        assert_eq!(cards.len(), 1);
        assert!(!cards.is_empty());
        assert_eq!(cards.first(), cards.only());
        assert_eq!(cards.nth(0), cards.first());
        assert_eq!(cards.all().len(), 1);
        assert!(game.player(p1).cards_in(HAND).matching(MURDER).is_empty());
    }

    #[test]
    fn two_matching_cards_are_all_returned() {
        let (game, [p1, ..]) = MockGame::new([P1.hand([LIGHTNING_BOLT, LIGHTNING_BOLT]), P2]);
        let cards = game.player(p1).cards_in(HAND).matching(LIGHTNING_BOLT);
        assert_eq!(cards.len(), 2);
        assert_eq!(cards.all().len(), 2);
        assert_ne!(cards.first(), cards.nth(1));
    }

    #[test]
    fn only_rejects_no_matching_card() {
        let (game, [p1, ..]) = MockGame::new([P1, P2]);
        let error = game
            .player(p1)
            .cards_in(HAND)
            .matching(LIGHTNING_BOLT)
            .only()
            .unwrap_err();
        assert_eq!(
            error,
            CardsError::Missing {
                player: PlayerSeat(1),
                card: "Lightning Bolt".to_string(),
                zone: HAND,
            }
        );
    }

    #[test]
    fn only_rejects_two_matching_cards() {
        let (game, [p1, ..]) = MockGame::new([P1.hand([LIGHTNING_BOLT, LIGHTNING_BOLT]), P2]);
        let error = game
            .player(p1)
            .cards_in(HAND)
            .matching(LIGHTNING_BOLT)
            .only()
            .unwrap_err();
        assert_eq!(
            error,
            CardsError::NotOne {
                player: PlayerSeat(1),
                card: "Lightning Bolt".to_string(),
                zone: HAND,
                count: 2,
            }
        );
    }

    #[test]
    fn first_and_nth_report_a_missing_card() {
        let (game, [p1, ..]) = MockGame::new([P1.hand([LIGHTNING_BOLT]), P2]);
        let cards = game.player(p1).cards_in(HAND).matching(LIGHTNING_BOLT);
        assert_eq!(
            cards.nth(3),
            Err(CardsError::OutOfRange {
                player: PlayerSeat(1),
                card: "Lightning Bolt".to_string(),
                zone: HAND,
                count: 1,
                index: 3,
            })
        );
        let missing = game.player(p1).cards_in(HAND).matching(MURDER);
        assert_eq!(
            missing.first(),
            Err(CardsError::Missing {
                player: PlayerSeat(1),
                card: "Murder".to_string(),
                zone: HAND,
            })
        );
    }
}
