use bevy::prelude::{Commands, Entity, Query, ResMut, Without};

use crate::StateBasedActionsPerformed;
use crate::players;

/// 704.5a If a player has 0 or less life, that player loses the game.
pub fn r704_5a_zero_life(
    mut commands: Commands,
    mut state_based_actions_performed: ResMut<StateBasedActionsPerformed>,
    players: Query<(Entity, &players::Life), Without<players::Lost>>,
) {
    for (player, life) in players.iter() {
        if **life <= 0 {
            commands.entity(player).insert(players::Lost);
            **state_based_actions_performed = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::mana;
    use crate::testing::*;

    #[test]
    fn bolt_to_zero_life_loses_the_game() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]), P2.life(3)]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        game.play([
            p1.cast(bolt).targeting(p2).pass_priority(),
            p2.pass_priority(),
        ])?;
        assert!(game.has_lost(p2));
        Ok(())
    }
}
