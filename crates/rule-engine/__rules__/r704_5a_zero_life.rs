use bevy::prelude::{Commands, Entity, Query, Without};

use crate::players;

/// 704.5a If a player has 0 or less life, that player loses the game.
pub fn r704_5a_zero_life(
    mut commands: Commands,
    players: Query<(Entity, &players::Life), Without<players::Lost>>,
) {
    for (player, life) in players.iter() {
        if **life <= 0 {
            commands.entity(player).insert(players::Lost);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::mana;
    use crate::players;
    use crate::testing::*;

    #[test]
    fn test_usage() {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]), P2.life(3)]);
        let [bolt] =
            game.player(p1).cards_in(HAND).matching(LIGHTNING_BOLT).exactly::<1>().unwrap();
        game.play([
            p1.cast(bolt).targeting(p2).pass_priority(),
            p2.pass_priority(),
        ])
        .unwrap();
        assert!(game.app.world().get::<players::Lost>(*p2).is_some());
    }
}
