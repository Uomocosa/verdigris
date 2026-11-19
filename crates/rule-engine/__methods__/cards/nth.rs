use crate::testing;

pub fn nth(cards: &testing::Cards, index: usize) -> Result<testing::TestCard, testing::CardsError> {
    let card = cards.all().get(index).copied();
    card.ok_or_else(|| testing::CardsError::OutOfRange {
        player: cards.seat(),
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
        assert_eq!(nth(&cards, 0), cards.only());
    }
}
