use crate::testing;

pub fn nth(cards: &testing::Cards, index: usize) -> Result<testing::TestCard, testing::CardsError> {
    let card = cards.all().get(index).copied();
    card.ok_or_else(|| testing::CardsError::OutOfRange {
        owner: cards.owner(),
        card: cards.card().to_string(),
        zone: cards.zone(),
        count: cards.len(),
        index,
    })
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::testing::*;

    use super::nth;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::new([P1.hand([LIGHTNING_BOLT]), P2]);
        let cards = game.player(p1).cards_in(HAND).matching(LIGHTNING_BOLT);
        assert_eq!(nth(&cards, 0).map(|card| [card]), cards.exactly::<1>());
    }

    #[test]
    fn out_of_range_index_is_reported() {
        let (game, [p1, ..]) = MockGame::new([P1.hand([LIGHTNING_BOLT]), P2]);
        let cards = game.player(p1).cards_in(HAND).matching(LIGHTNING_BOLT);
        assert_eq!(
            nth(&cards, 3),
            Err(CardsError::OutOfRange {
                owner: p1.into(),
                card: "Lightning Bolt".to_string(),
                zone: HAND,
                count: 1,
                index: 3,
            })
        );
    }
}
