use atomic_delegate_macros::atomic_delegates;

use crate::cards;
use crate::testing;
use crate::zones;

pub struct CardsInZone {
    pub(crate) cards: Vec<(testing::TestCard, cards::CardName)>,
    zone: zones::Zone,
    player: testing::TestPlayer,
}

impl CardsInZone {
    pub(crate) const fn from_cards(
        cards: Vec<(testing::TestCard, cards::CardName)>,
        zone: zones::Zone,
        player: testing::TestPlayer,
    ) -> Self {
        Self {
            cards,
            zone,
            player,
        }
    }
}

#[rustfmt::skip]
impl CardsInZone {
    #[must_use]
    pub const fn is_empty(&self) -> bool { self.cards.is_empty() }

    #[must_use]
    pub const fn len(&self) -> usize { self.cards.len() }

    #[must_use]
    pub fn has_no(&self, card: impl Into<testing::CardQuery>) -> bool { !self.has(card) }
}

impl CardsInZone {
    #[must_use]
    pub fn matching(&self, card: impl Into<testing::CardQuery>) -> testing::Cards {
        let card = card.into();
        let matching = self.matching_cards(card);
        testing::Cards::from_matches(matching, card.label(), self.zone, self.player)
    }

    pub fn exactly<const N: usize>(
        &self,
        card: impl Into<testing::CardQuery>,
    ) -> Result<[testing::TestCard; N], testing::CardsError> {
        let matching = self.matching(card);
        matching.exactly::<N>()
    }
}

#[atomic_delegates]
impl CardsInZone {
    pub fn has(&self, card: impl Into<testing::CardQuery>) -> bool {}
}

#[atomic_delegates]
impl CardsInZone {
    pub(crate) fn matching_cards(&self, card: testing::CardQuery) -> Vec<testing::TestCard> {}
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::HAND;
    use crate::testing::*;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::new([P1.hand([LIGHTNING_BOLT, LIGHTNING_BOLT]), P2]);
        let cards = game.player(p1).cards_in(HAND);

        assert_eq!(cards.len(), 2);
        assert!(!cards.is_empty());
        assert!(cards.has(LIGHTNING_BOLT));
        assert!(cards.has_no(MURDER));
        assert_eq!(cards.matching(LIGHTNING_BOLT).len(), 2);
        assert!(cards.matching(MURDER).is_empty());
        assert!(cards.exactly::<1>(LIGHTNING_BOLT).is_err());
    }

    #[test]
    fn exactly_returns_the_single_matching_card() {
        let (game, [p1, ..]) = MockGame::new([P1.hand([LIGHTNING_BOLT]), P2]);
        let cards = game.player(p1).cards_in(HAND);
        let [found] = cards.exactly::<1>(LIGHTNING_BOLT).expect("P1 holds one Lightning Bolt");

        assert!(cards.has(found));
        assert_eq!(cards.exactly::<1>(found), Ok([found]));
    }

    #[test]
    fn an_empty_zone_has_no_cards() {
        let (game, [p1, ..]) = MockGame::new([P1, P2]);
        let cards = game.player(p1).cards_in(BATTLEFIELD);

        assert!(cards.is_empty());
        assert_eq!(cards.len(), 0);
        assert!(cards.has_no(GRIZZLY_BEARS));
        assert_eq!(
            cards.exactly::<1>(GRIZZLY_BEARS),
            Err(CardsError::WrongCount {
                player: PlayerSeat(1),
                card: "Grizzly Bears".to_string(),
                zone: BATTLEFIELD,
                count: 0,
                expected: 1,
            })
        );
    }
}
