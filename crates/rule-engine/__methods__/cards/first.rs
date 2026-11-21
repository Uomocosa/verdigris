use crate::testing;

pub fn first(cards: &testing::Cards) -> Result<testing::TestCard, testing::CardsError> {
    let card = cards.all().first().copied();
    card.ok_or_else(|| testing::CardsError::Missing {
        owner: cards.owner(),
        card: cards.card().to_string(),
        zone: cards.zone(),
    })
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::testing::*;

    use super::first;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::new([P1.hand([LIGHTNING_BOLT]), P2]);
        let cards = game.player(p1).cards_in(HAND).matching(LIGHTNING_BOLT);
        assert_eq!(first(&cards).map(|card| [card]), cards.exactly::<1>());
    }

    #[test]
    fn missing_card_is_reported() {
        let (game, [p1, ..]) = MockGame::new([P1.hand([LIGHTNING_BOLT]), P2]);
        let missing = game.player(p1).cards_in(HAND).matching(MURDER);
        assert_eq!(
            first(&missing),
            Err(CardsError::Missing {
                owner: p1.into(),
                card: "Murder".to_string(),
                zone: HAND,
            })
        );
    }
}
