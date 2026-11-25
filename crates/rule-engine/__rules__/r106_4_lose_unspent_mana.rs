use bevy::prelude::{MessageReader, Query};

use crate::Resolve;
use crate::cards;
use crate::events;
use crate::rules::traits::context::Context;

/// 106.4. When an effect instructs a player to add mana, that mana goes into a player’s mana pool.
/// From there, it can be used to pay costs immediately, or it can stay in the player’s mana pool as
/// unspent mana. Each player’s mana pool empties at the end of each step and phase, and the player
/// is said to lose this mana. Cards with abilities that produce mana or refer to unspent mana have
/// received errata in the Oracle™ card reference to no longer explicitly refer to the mana pool.
pub fn r106_4_lose_unspent_mana(
    mut losses: MessageReader<Resolve<events::LoseUnspentMana>>,
    mut pools: Query<&mut cards::ManaPool>,
) {
    for loss in losses.read() {
        if let Ok(mut pool) = pools.get_mut(*loss.player()) {
            *pool = cards::ManaPool::EMPTY;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::mana;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn unspent_mana_empties_when_the_step_ends() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.mana_pool(mana![R, 2, U]), P2]);
        assert_eq!(game.mana_pool(p1), mana![R, 2, U]);

        game.play([p1.pass_to(turns::Step::End), p2.pass_to(turns::Step::End)])?;

        assert_eq!(game.mana_pool(p1), cards::ManaPool::EMPTY);
        assert_eq!(game.mana_pool(p2), cards::ManaPool::EMPTY);
        Ok(())
    }
}
