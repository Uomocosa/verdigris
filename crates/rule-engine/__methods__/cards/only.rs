use bevy::prelude::Entity;

use crate::testing;

pub fn only(cards: &testing::Cards) -> Entity {
    cards.assert_exactly_one();
    cards.first().unwrap_or(Entity::PLACEHOLDER)
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::testing::*;

    use super::only;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::new([P1.hand([LIGHTNING_BOLT]), P2]);
        let cards = game.player(p1).cards(HAND, LIGHTNING_BOLT);
        assert_eq!(Some(only(&cards)), cards.first());
    }
}
