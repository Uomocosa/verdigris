use bevy::prelude::{MessageReader, MessageWriter, Res};

use crate::turns;
use crate::zones;

/// 504.1 First, the active player draws a card. This turn-based action doesn’t use the stack.
pub fn r504_1_draw_step(
    mut step_started: MessageReader<turns::StepStarted>,
    mut draws: MessageWriter<zones::DrawRequested>,
    turn: Res<turns::Turn>,
) {
    let turn = turn.into_inner();
    for step in step_started.read() {
        if **step == turns::Step::Draw {
            draws.write(zones::DrawRequested {
                player: turn.active,
                count: 1,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::testing;
    use crate::turns;
    use crate::zones;

    #[test]
    fn test_usage() {
        let mut game = testing::MockGame::builder()
            .turn(1, testing::Seat::P1, turns::Step::Upkeep)
            .build();

        game.pass_priority_all();
        game.pass_priority();

        assert_eq!(
            game.app.world().resource::<turns::Turn>().step,
            turns::Step::Draw
        );
    }

    #[test]
    fn the_active_player_draws_a_card_during_their_draw_step() {
        let mut game = testing::MockGame::builder()
            .turn(1, testing::Seat::P1, turns::Step::Upkeep)
            .library(
                testing::Seat::P1,
                [testing::grizzly_bears(), testing::lightning_bolt()],
            )
            .build();
        let top = game
            .find(game.p1, zones::Zone::Library, "Grizzly Bears")
            .unwrap();

        game.pass_priority_all();
        game.pass_priority();

        assert_eq!(game.zone_of(top), zones::Zone::Hand);
        assert_eq!(
            game.find(game.p1, zones::Zone::Hand, "Grizzly Bears")
                .unwrap(),
            top
        );
    }
}
