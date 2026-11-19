use crate::players;
use crate::testing;

pub fn life(game: &testing::MockGame, player: impl Into<bevy::prelude::Entity>) -> i32 {
    game.app
        .world()
        .get::<players::Life>(player.into())
        .map_or(0, |life| **life)
}

#[cfg(test)]
mod tests {
    use crate::testing::*;

    use super::life;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::new([P1, P2]);
        assert_eq!(life(&game, p1), 20);
    }
}
