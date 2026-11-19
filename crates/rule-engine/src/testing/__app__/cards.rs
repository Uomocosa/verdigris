use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::Entity;

use crate::testing;
use crate::zones;

pub struct Cards {
    entities: Vec<Entity>,
    card_name: &'static str,
    zone: zones::Zone,
    seat: usize,
}

impl Cards {
    pub(crate) const fn from_matches(
        entities: Vec<Entity>,
        card_name: &'static str,
        zone: zones::Zone,
        seat: usize,
    ) -> Self {
        Self {
            entities,
            card_name,
            zone,
            seat,
        }
    }
}

#[rustfmt::skip]
impl Cards {
    #[must_use]
    pub fn all(&self) -> &[Entity] { &self.entities }

    #[must_use]
    pub const fn is_empty(&self) -> bool { self.entities.is_empty() }

    #[must_use]
    pub const fn len(&self) -> usize { self.entities.len() }
}

#[atomic_delegates]
impl Cards {
    pub fn first(&self) -> Result<Entity, testing::CardsError> {}

    pub fn nth(&self, index: usize) -> Result<Entity, testing::CardsError> {}
}

impl Cards {
    pub const fn only(&self) -> Result<Entity, testing::CardsError> {
        match self.entities.as_slice() {
            [entity] => Ok(*entity),
            [] => Err(testing::CardsError::Missing {
                player: self.seat,
                card: self.card_name,
                zone: self.zone,
            }),
            _ => Err(testing::CardsError::NotOne {
                player: self.seat,
                card: self.card_name,
                zone: self.zone,
                count: self.entities.len(),
            }),
        }
    }
}

#[rustfmt::skip]
impl Cards {
    pub(crate) const fn card_name(&self) -> &'static str { self.card_name }

    pub(crate) const fn seat(&self) -> usize { self.seat }

    pub(crate) const fn zone(&self) -> zones::Zone { self.zone }
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::testing::*;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::new([P1.hand([LIGHTNING_BOLT]), P2]);
        let cards = game.player(p1).cards(HAND, LIGHTNING_BOLT);
        assert_eq!(cards.len(), 1);
        assert!(!cards.is_empty());
        assert_eq!(cards.first(), cards.only());
        assert_eq!(cards.nth(0), cards.first());
        assert_eq!(cards.all().len(), 1);
        assert!(game.player(p1).cards(HAND, MURDER).is_empty());
    }

    #[test]
    fn two_matching_cards_are_all_returned() {
        let (game, [p1, ..]) = MockGame::new([P1.hand([LIGHTNING_BOLT, LIGHTNING_BOLT]), P2]);
        let cards = game.player(p1).cards(HAND, LIGHTNING_BOLT);
        assert_eq!(cards.len(), 2);
        assert_eq!(cards.all().len(), 2);
        assert_ne!(cards.first(), cards.nth(1));
    }

    #[test]
    fn only_rejects_no_matching_card() {
        let (game, [p1, ..]) = MockGame::new([P1, P2]);
        let error = game
            .player(p1)
            .cards(HAND, LIGHTNING_BOLT)
            .only()
            .unwrap_err();
        assert_eq!(
            error,
            CardsError::Missing {
                player: 1,
                card: "Lightning Bolt",
                zone: HAND,
            }
        );
    }

    #[test]
    fn only_rejects_two_matching_cards() {
        let (game, [p1, ..]) = MockGame::new([P1.hand([LIGHTNING_BOLT, LIGHTNING_BOLT]), P2]);
        let error = game
            .player(p1)
            .cards(HAND, LIGHTNING_BOLT)
            .only()
            .unwrap_err();
        assert_eq!(
            error,
            CardsError::NotOne {
                player: 1,
                card: "Lightning Bolt",
                zone: HAND,
                count: 2,
            }
        );
    }

    #[test]
    fn first_and_nth_report_a_missing_card() {
        let (game, [p1, ..]) = MockGame::new([P1.hand([LIGHTNING_BOLT]), P2]);
        let cards = game.player(p1).cards(HAND, LIGHTNING_BOLT);
        assert_eq!(
            cards.nth(3),
            Err(CardsError::OutOfRange {
                player: 1,
                card: "Lightning Bolt",
                zone: HAND,
                count: 1,
                index: 3,
            })
        );
        let missing = game.player(p1).cards(HAND, MURDER);
        assert_eq!(
            missing.first(),
            Err(CardsError::Missing {
                player: 1,
                card: "Murder",
                zone: HAND,
            })
        );
    }
}
