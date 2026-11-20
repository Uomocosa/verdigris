use crate::testing;

pub fn has(cards: &testing::CardsInZone, card: impl Into<testing::CardQuery>) -> bool {
    let card = card.into();
    cards.cards.iter().any(|(test_card, name)| card.matches(*test_card, name))
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::testing::*;

    use super::has;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::new([P1.hand([LIGHTNING_BOLT]), P2]);
        let cards = game.player(p1).cards_in(HAND);

        assert!(has(&cards, LIGHTNING_BOLT));
        assert!(!has(&cards, MURDER));
    }
}
