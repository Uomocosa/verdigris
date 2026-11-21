use crate::testing;

pub fn exactly_trigger<const N: usize>(
    cards: &testing::CardsInZone,
    query: testing::TriggerQuery,
) -> Result<[testing::TestTrigger; N], testing::CardsError> {
    let found: Vec<testing::TestTrigger> =
        cards.objects.iter().filter_map(|object| query.matches(object)).collect();
    <[testing::TestTrigger; N]>::try_from(found).map_err(|_| testing::CardsError::WrongCount {
        owner: cards.owner(),
        card: query.label(),
        zone: cards.zone(),
        count: cards.count(),
        expected: N,
    })
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::mana;
    use crate::testing::*;

    use super::exactly_trigger;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([SOUL_WARDEN]).hand([GRIZZLY_BEARS]).mana_pool(mana![1, G]),
            P2,
        ]);
        let [bears] = game.player(p1).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([p1.cast(bears).pass_priority(), p2.pass_priority()])?;
        let stack = game.stack();
        let [trigger] = exactly_trigger::<1>(&stack, SOUL_WARDEN.trigger())?;
        assert_eq!(
            stack.exactly_trigger::<1>(SOUL_WARDEN.trigger()),
            Ok([trigger])
        );
        assert!(stack.exactly_trigger::<1>(MURDER.trigger()).is_err());
        Ok(())
    }
}
