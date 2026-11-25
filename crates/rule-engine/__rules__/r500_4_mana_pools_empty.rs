use bevy::ecs::schedule::{IntoScheduleConfigs, ScheduleConfigs};
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::{MessageWriter, on_message};

use crate::Attempt;
use crate::events;
use crate::players;
use crate::turns;

/// 500.4. When a step or phase ends, any unused mana left in a player’s mana pool empties. This
/// turn-based action doesn’t use the stack.
pub fn r500_4_mana_pools_empty() -> ScheduleConfigs<ScheduleSystem> {
    mana_pools_empty.run_if(on_message::<turns::StepStarted>)
}

fn mana_pools_empty(
    mut losses: MessageWriter<Attempt<events::LoseUnspentMana>>,
    players: players::Players,
) {
    for player in players.still_in_the_game() {
        losses.write(Attempt(events::LoseUnspentMana(player.id)));
    }
}

#[cfg(test)]
mod tests {
    use crate::mana;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn unspent_mana_stays_when_players_dont_lose_it() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([UPWELLING]).mana_pool(mana![R, 2, U]),
            P2.mana_pool(mana![G]),
        ]);

        game.play([p1.pass_to(turns::Step::End), p2.pass_to(turns::Step::End)])?;

        assert_eq!(game.mana_pool(p1), mana![R, 2, U]);
        assert_eq!(game.mana_pool(p2), mana![G]);
        Ok(())
    }
}
