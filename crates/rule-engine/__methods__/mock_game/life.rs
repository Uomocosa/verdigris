use crate::players;
use crate::testing;

pub fn life(game: &testing::MockGame, player: bevy::prelude::Entity) -> i32 {
    game.app
        .world()
        .get::<players::Life>(player)
        .map_or(0, |life| **life)
}

#[cfg(test)]
mod tests {
    use crate::testing::*;

    use super::life;

    #[test]
    fn test_usage() {
        let game = MockGame::builder().build();
        assert_eq!(life(&game, game.p1), 20);
    }
}
