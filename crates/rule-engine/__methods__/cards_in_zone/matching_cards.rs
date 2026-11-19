use crate::testing;

pub fn matching_cards(
    cards: &testing::CardsInZone,
    card: testing::CardQuery,
) -> Vec<testing::TestCard> {
    cards
        .cards
        .iter()
        .filter(|(test_card, name)| card.matches(*test_card, name))
        .map(|(test_card, _)| *test_card)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::testing::*;

    use super::matching_cards;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::new([P1.hand([LIGHTNING_BOLT, MURDER]), P2]);
        let cards = game.player(p1).cards_in(HAND);
        let matching = matching_cards(&cards, CardQuery::from(LIGHTNING_BOLT));

        assert_eq!(matching.len(), 1);
    }
}
