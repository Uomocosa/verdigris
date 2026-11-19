use bevy::prelude::{MessageReader, MessageWriter, Res};

use crate::players;
use crate::turns;

/// 504.1 First, the active player draws a card. This turn-based action doesn’t use the stack.
pub fn r504_1_draw_step(
    mut step_started: MessageReader<turns::StepStarted>,
    mut draws: MessageWriter<players::DrawRequested>,
    turn: Res<turns::Turn>,
) {
    let turn = turn.into_inner();
    for step in step_started.read() {
        if **step == turns::Step::Draw {
            draws.write(players::DrawRequested {
                player: turn.active,
                count: 1,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;
    use crate::turns;
    use crate::{HAND, LIBRARY};

    #[test]
    fn test_usage() {
        let (mut game, [_, _]) = MockGame::builder()
            .turn(1, turns::Step::Upkeep)
            .players([P1, P2]);

        game.pass_priority_all();
        game.pass_priority();

        assert_eq!(
            game.app.world().resource::<turns::Turn>().step,
            turns::Step::Draw
        );
    }

    #[test]
    fn the_active_player_draws_a_card_during_their_draw_step() {
        let (mut game, [p1, ..]) = MockGame::builder()
            .turn(1, turns::Step::Upkeep)
            .players([P1.library([grizzly_bears(), lightning_bolt()]), P2]);
        let top = game
            .player(p1)
            .cards(LIBRARY, GRIZZLY_BEARS)
            .only()
            .unwrap();

        game.pass_priority_all();
        game.pass_priority();

        assert_eq!(game.zone_of(top), HAND);
        assert_eq!(
            game.player(p1).cards(HAND, GRIZZLY_BEARS).only().unwrap(),
            top
        );
    }
}
