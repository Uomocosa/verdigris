use bevy::ecs::schedule::{IntoScheduleConfigs, ScheduleConfigs};
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::MessageWriter;

use crate::Attempt;
use crate::Game;
use crate::events;
use crate::turns;

/// 504.1. First, the active player draws a card. This turn-based action doesn’t use the stack.
pub fn r504_1_draw_step() -> ScheduleConfigs<ScheduleSystem> {
    draw_a_card.run_if(turns::Step::Draw.begins())
}

fn draw_a_card(mut draws: MessageWriter<Attempt<events::Draw>>, game: Game) {
    if let Some(active_player) = game.active_player() {
        draws.write(Attempt(events::Draw {
            player: active_player.id,
            amount: 1,
        }));
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;
    use crate::turns;
    use crate::{HAND, LIBRARY};

    #[test]
    fn the_active_player_draws_a_card_during_their_draw_step() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::builder()
            .turn(2, turns::Step::Upkeep)
            .players([P1.library([GRIZZLY_BEARS, LIGHTNING_BOLT]), P2]);
        let [bears] = game.player(p1).cards_in(LIBRARY).exactly::<1>(GRIZZLY_BEARS)?;

        game.play([p1.pass_priority(), p2.pass_priority(), p1.pass_priority()])?;

        assert!(game.player(p1).cards_in(HAND).has(bears));
        Ok(())
    }
}
