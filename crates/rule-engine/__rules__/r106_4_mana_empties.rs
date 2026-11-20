use bevy::prelude::{MessageReader, Query};

use crate::cards;
use crate::turns;

/// 106.4 When an effect instructs a player to add mana, that mana goes into a player's mana
/// pool. From there, it can be used to pay costs immediately, or it can stay in the player's
/// mana pool as unspent mana. Each player's mana pool empties at the end of each step and
/// phase, and the player is said to lose this mana. Cards with abilities that produce mana or
/// refer to unspent mana have received errata in the Oracle™ card reference to no longer
/// explicitly refer to the mana pool.
pub fn r106_4_mana_empties(
    mut started: MessageReader<turns::StepStarted>,
    mut pools: Query<&mut cards::ManaPool>,
) {
    for _ in started.read() {
        for mut pool in pools.iter_mut() {
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
    fn test_usage() {
        let (mut game, [p1, p2]) = MockGame::new([P1.mana_pool(mana![R]), P2]);

        game.play([p1.pass_priority(), p2.pass_priority()]).unwrap();

        assert_eq!(
            game.app.world().resource::<turns::Turn>().step,
            turns::Step::BeginningOfCombat
        );
    }

    #[test]
    fn r106_4_unspent_mana_empties_when_the_step_ends() {
        let (mut game, [p1, p2]) = MockGame::new([P1.mana_pool(mana![R, 2, U]), P2]);
        assert_eq!(game.mana_pool(p1), mana![R, 2, U]);

        game.play([p1.pass_to(turns::Step::End), p2.pass_to(turns::Step::End)]).unwrap();

        assert_eq!(game.mana_pool(p1), cards::ManaPool::EMPTY);
        assert_eq!(game.mana_pool(p2), cards::ManaPool::EMPTY);
    }
}
