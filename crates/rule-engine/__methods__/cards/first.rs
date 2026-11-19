use bevy::prelude::Entity;

use crate::testing;

pub fn first(cards: &testing::Cards) -> Result<Entity, testing::CardsError> {
    let entity = cards.all().first().copied();
    entity.ok_or_else(|| testing::CardsError::Missing {
        player: cards.seat(),
        card: cards.card_name(),
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
        let cards = game.player(p1).cards(HAND, LIGHTNING_BOLT);
        assert_eq!(first(&cards), cards.only());
    }
}
