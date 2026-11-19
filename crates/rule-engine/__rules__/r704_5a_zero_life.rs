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
    use crate::actions;
    use crate::mana;
    use crate::testing;
    use crate::zones;

    #[test]
    fn test_usage() {
        let mut game = testing::MockGame::builder()
            .life(testing::Seat::P2, 3)
            .hand(testing::Seat::P1, [testing::lightning_bolt()])
            .mana_pool(testing::Seat::P1, mana![R])
            .build();
        let bolt = game
            .find(game.p1, zones::Zone::Hand, "Lightning Bolt")
            .unwrap();
        let p2 = game.p2;
        game.act(game.p1, actions::Action::Cast(bolt));
        game.act(game.p1, actions::Action::ChooseTargets(vec![p2]));
        game.pass_priority_all();
        assert!(game.app.world().get::<zones::Lost>(p2).is_some());
    }
}
