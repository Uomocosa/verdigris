use crate::cards;
use crate::testing;

pub fn mana_pool(game: &testing::MockGame, player: bevy::prelude::Entity) -> cards::ManaPool {
    game.app
        .world()
        .get::<cards::ManaPool>(player)
        .copied()
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use crate::mana;
    use crate::testing;

    use super::mana_pool;

    #[test]
    fn test_usage() {
        let game = testing::MockGame::builder()
            .mana_pool(testing::Seat::P1, mana![R])
            .build();
        assert_eq!(mana_pool(&game, game.p1), mana![R]);
    }
}
