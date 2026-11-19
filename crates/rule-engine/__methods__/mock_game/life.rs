use crate::testing;
use crate::zones;

pub fn life(game: &testing::MockGame, player: bevy::prelude::Entity) -> i32 {
    game.app
        .world()
        .get::<zones::Life>(player)
        .map_or(0, |life| **life)
}

#[cfg(test)]
mod tests {
    use crate::testing;

    use super::life;

    #[test]
    fn test_usage() {
        let game = testing::MockGame::builder().build();
        assert_eq!(life(&game, game.p1), 20);
    }
}
