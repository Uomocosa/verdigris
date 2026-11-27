use atomic_delegate_macros::atomic_delegates;

use crate::testing;
use crate::zones;

pub struct CardsInZone {
    pub(crate) objects: Vec<testing::ZoneObject>,
    zone: zones::Zone,
    owner: testing::ZoneOwner,
}

impl CardsInZone {
    pub(crate) const fn from_objects(
        objects: Vec<testing::ZoneObject>,
        zone: zones::Zone,
        owner: testing::ZoneOwner,
    ) -> Self {
        Self {
            objects,
            zone,
            owner,
        }
    }
}

#[rustfmt::skip]
impl CardsInZone {
    #[must_use]
    pub const fn is_empty(&self) -> bool { self.objects.is_empty() }

    #[must_use]
    pub const fn count(&self) -> usize { self.objects.len() }

    #[must_use]
    pub fn has_no(&self, card: testing::Card) -> bool { !self.has(card) }

    #[must_use]
    pub fn has_no_trigger(&self, query: testing::TriggerQuery) -> bool { !self.has_trigger(query) }

    #[must_use]
    pub fn has_trigger(&self, query: testing::TriggerQuery) -> bool { self.objects.iter().any(|object| query.matches(object).is_some()) }

    pub fn exactly<const N: usize>(&self, card: testing::Card) -> Result<[testing::TestCard; N], testing::CardsError> { self.matching(card).exactly::<N>() }
}

impl CardsInZone {
    #[must_use]
    pub fn has(&self, card: testing::Card) -> bool {
        let query = testing::CardQuery::from(card);
        self.objects.iter().any(|object| query.matches(object).is_some())
    }

    #[must_use]
    pub fn matching(&self, card: impl Into<testing::CardQuery>) -> testing::Cards {
        let query: testing::CardQuery = card.into();
        let found: Vec<testing::TestCard> =
            self.objects.iter().filter_map(|object| query.matches(object)).collect();
        testing::Cards::from_matches(found, query.label(), self.zone, self.owner)
    }
}

#[atomic_delegates]
impl CardsInZone {
    pub fn exactly_trigger<const N: usize>(
        &self,
        query: testing::TriggerQuery,
    ) -> Result<[testing::TestTrigger; N], testing::CardsError> {
    }
}

#[rustfmt::skip]
impl CardsInZone {
    pub(crate) const fn owner(&self) -> testing::ZoneOwner { self.owner }

    pub(crate) const fn zone(&self) -> zones::Zone { self.zone }
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

        assert_eq!(cards.count(), 2);
        assert!(!cards.is_empty());
        assert!(cards.has(LIGHTNING_BOLT));
        assert!(cards.has_no(MURDER));
        assert_eq!(cards.matching(LIGHTNING_BOLT).len(), 2);
        assert!(cards.matching(MURDER).is_empty());
        assert!(cards.exactly::<1>(LIGHTNING_BOLT).is_err());
    }

    #[test]
    fn exactly_returns_the_single_matching_card() -> TestResult {
        let (game, [p1, ..]) = MockGame::new([P1.hand([LIGHTNING_BOLT]), P2]);
        let cards = game.player(p1).cards_in(HAND);
        let [found] = cards.exactly::<1>(LIGHTNING_BOLT)?;

        assert!(game.card(found).is_in(HAND).of(p1));
        Ok(())
    }

    #[test]
    fn an_empty_zone_has_no_cards() {
        let (game, [p1, ..]) = MockGame::new([P1, P2]);
        let cards = game.player(p1).cards_in(BATTLEFIELD);

        assert!(cards.is_empty());
        assert_eq!(cards.count(), 0);
        assert!(cards.has_no(GRIZZLY_BEARS));
        assert_eq!(
            cards.exactly::<1>(GRIZZLY_BEARS),
            Err(CardsError::WrongCount {
                owner: p1.into(),
                card: "Grizzly Bears".to_string(),
                zone: BATTLEFIELD,
                count: 0,
                expected: 1,
            })
        );
    }
}
