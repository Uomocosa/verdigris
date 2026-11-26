use crate::events;
use bevy::prelude::{Entity, MessageWriter, Query};

use crate::Attempt;
use crate::players;

/// 704.5a If a player has 0 or less life, that player loses the game.
pub fn r704_5a_zero_life(
    mut losses: MessageWriter<Attempt<events::LoseGame>>,
    players: Query<(Entity, &players::Life), players::StillInTheGame>,
) {
    for (player, life) in players.iter() {
        if **life <= 0 {
            losses.write(Attempt(events::LoseGame {
                player: players::PlayerId(player),
            }));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
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

    #[test]
    fn zero_life_does_not_lose_while_platinum_angel_is_on_the_battlefield() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([PLATINUM_ANGEL]).life(0),
            P2.hand([DOOM_BLADE]).mana_pool(mana![1, B]),
        ]);
        let [angel] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(PLATINUM_ANGEL)?;
        let [doom_blade] = game.player(p2).cards_in(HAND).exactly::<1>(DOOM_BLADE)?;
        game.pass_priority();
        assert!(!game.has_lost(p1));
        game.play([
            p2.cast(doom_blade).targeting(angel).pass_priority(),
            p1.pass_priority(),
        ])?;
        assert!(game.has_lost(p1));
        Ok(())
    }
}
