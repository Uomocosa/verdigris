use crate::testing;
use crate::zones;

pub fn is_on_the_stack(check: testing::CardCheck<'_>) -> bool {
    let testing::CardCheck { game, card } = check;
    game.app.world().get::<zones::InZone>(*card).is_some_and(|in_zone| **in_zone == game.stack)
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::mana;
    use crate::testing::*;

    use super::is_on_the_stack;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, ..]) =
            MockGame::new([P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]), P2]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;

        assert!(!is_on_the_stack(game.card(bolt)));
        game.play([p1.cast(bolt)])?;
        let [bolt_on_stack] = game.stack().exactly::<1>(LIGHTNING_BOLT)?;
        assert!(is_on_the_stack(game.card(bolt_on_stack)));
        Ok(())
    }
}
