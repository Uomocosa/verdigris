use crate::cards;
use crate::testing;

pub fn mana_pool(
    game: &testing::MockGame,
    player: impl Into<bevy::prelude::Entity>,
) -> cards::ManaPool {
    game.app
        .world()
        .get::<cards::ManaPool>(player.into())
        .copied()
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use crate::mana;
    use crate::testing::*;

    use super::mana_pool;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::new([P1.mana_pool(mana![R]), P2]);
        assert_eq!(mana_pool(&game, p1), mana![R]);
    }
}
