use bevy::prelude::{Entity, MessageWriter, Query, With};

use crate::players;
use crate::rules;

/// 704.5b If a player attempted to draw a card from a library with no cards in it since the last
/// time state-based actions were checked, that player loses the game.
pub fn r704_5b_empty_library(
    mut losses: MessageWriter<rules::Would<rules::LoseGame>>,
    players: Query<
        Entity,
        (
            With<players::AttemptedDrawFromEmptyLibrary>,
            players::StillInTheGame,
        ),
    >,
) {
    for player in players.iter() {
        losses.write(rules::Would(rules::LoseGame(player)));
    }
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::mana;
    use crate::testing::*;

    #[test]
    fn drawing_from_an_empty_library_loses_the_game_at_the_next_state_based_check() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([DIVINATION]).empty_library().mana_pool(mana![2, U]),
            P2,
        ]);
        let [divination] = game.player(p1).cards_in(HAND).exactly::<1>(DIVINATION)?;
        game.play([p1.cast(divination)])?;
        assert!(!game.has_lost(p1));
        game.play([p1.pass_priority(), p2.pass_priority()])?;
        assert!(game.has_lost(p1));
        Ok(())
    }
}
