use bevy::prelude::{Commands, Entity, Query, Without};

use crate::zones;

/// 704.5a If a player has 0 or less life, that player loses the game.
pub fn r704_5a_zero_life(
    mut commands: Commands,
    players: Query<(Entity, &zones::Life), Without<zones::Lost>>,
) {
    for (player, life) in players.iter() {
        if **life <= 0 {
            commands.entity(player).insert(zones::Lost);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::testing;
    use crate::zones;

    #[test]
    fn test_usage() {
        let mut game = testing::MockGame::builder().build();
        let p2 = game.p2;
        game.app.world_mut().entity_mut(p2).insert(zones::Life(0));
        game.pass_priority_all();
        assert!(game.app.world().get::<zones::Lost>(p2).is_some());
    }
}
