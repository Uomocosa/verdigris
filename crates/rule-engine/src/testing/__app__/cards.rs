use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::Entity;

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
    pub fn first(&self) -> Option<Entity> { self.entities.first().copied() }

    #[must_use]
    pub const fn is_empty(&self) -> bool { self.entities.is_empty() }

    #[must_use]
    pub const fn len(&self) -> usize { self.entities.len() }

    #[must_use]
    pub fn nth(&self, index: usize) -> Option<Entity> { self.entities.get(index).copied() }
}

#[atomic_delegates]
impl Cards {
    pub fn only(&self) -> Entity {}
}

#[rustfmt::skip]
impl Cards {
    pub(crate) fn assert_exactly_one(&self) { assert_eq!(self.entities.len(), 1, "{}", self.count_message()) }
}

impl Cards {
    pub(crate) fn count_message(&self) -> String {
        if self.entities.is_empty() {
            format!(
                "P{} has no {} in {:?}",
                self.seat, self.card_name, self.zone
            )
        } else {
            format!(
                "P{} has {} {} in {:?}, expected exactly one",
                self.seat,
                self.entities.len(),
                self.card_name,
                self.zone
            )
        }
    }
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
        assert_eq!(cards.first(), Some(cards.only()));
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
    #[should_panic(expected = "P1 has no Lightning Bolt in Hand")]
    fn only_rejects_no_matching_card() {
        let (game, [p1, ..]) = MockGame::new([P1, P2]);
        let _ = game.player(p1).cards(HAND, LIGHTNING_BOLT).only();
    }

    #[test]
    #[should_panic(expected = "P1 has 2 Lightning Bolt in Hand, expected exactly one")]
    fn only_rejects_two_matching_cards() {
        let (game, [p1, ..]) = MockGame::new([P1.hand([LIGHTNING_BOLT, LIGHTNING_BOLT]), P2]);
        let _ = game.player(p1).cards(HAND, LIGHTNING_BOLT).only();
    }
}
