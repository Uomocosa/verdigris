use bevy::prelude::Entity;

use crate::testing;
use crate::turns;

pub fn give_priority(game: &mut testing::MockGame, player: Entity) {
    game.app
        .world_mut()
        .insert_resource(turns::Priority(player));
}

#[cfg(test)]
mod tests {
    use crate::testing;
    use crate::turns;

    use super::give_priority;

    #[test]
    fn test_usage() {
        let mut game = testing::MockGame::builder().build();
        let p2 = game.p2;
        give_priority(&mut game, p2);
        assert_eq!(**game.app.world().resource::<turns::Priority>(), p2);
    }
}
