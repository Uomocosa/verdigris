use crate::testing;

pub fn exactly<const N: usize>(
    cards: &testing::Cards,
) -> Result<[testing::TestCard; N], testing::CardsError> {
    <[testing::TestCard; N]>::try_from(cards.all()).map_err(|_| testing::CardsError::WrongCount {
        owner: cards.owner(),
        card: cards.card().to_string(),
        zone: cards.zone(),
        count: cards.len(),
        expected: N,
    })
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::testing::*;

    use super::exactly;

    #[test]
    fn test_usage() -> TestResult {
        let (game, [p1, ..]) = MockGame::new([P1.hand([LIGHTNING_BOLT, LIGHTNING_BOLT]), P2]);
        let cards = game.player(p1).cards_in(HAND).matching(LIGHTNING_BOLT);
        let [first, second] = exactly(&cards)?;
        assert_eq!(Ok(first), cards.first());
        assert_eq!(Ok(second), cards.nth(1));
        Ok(())
    }

    #[test]
    fn wrong_count_is_reported() {
        let (game, [p1, ..]) = MockGame::new([P1.hand([LIGHTNING_BOLT]), P2]);
        let cards = game.player(p1).cards_in(HAND).matching(LIGHTNING_BOLT);
        assert_eq!(
            exactly::<2>(&cards),
            Err(CardsError::WrongCount {
                owner: p1.into(),
                card: "Lightning Bolt".to_string(),
                zone: HAND,
                count: 1,
                expected: 2,
            })
        );
    }
}
