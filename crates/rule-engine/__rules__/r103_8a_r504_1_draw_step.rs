use bevy::prelude::{Entity, MessageReader, MessageWriter, Query, Res, With};

use crate::players;
use crate::turns;

/// 504.1. First, the active player draws a card. This turn-based action doesn’t use the stack.
///
/// 103.8a In a two-player game, the player who plays first skips the draw step (see rule 504, “Draw
/// Step”) of their first turn.
pub fn r103_8a_r504_1_draw_step(
    mut step_started: MessageReader<turns::StepStarted>,
    mut draws: MessageWriter<players::DrawRequested>,
    turn: Res<turns::Turn>,
    turn_order: Res<turns::TurnOrder>,
    players: Query<Entity, With<players::Player>>,
) {
    let turn = turn.into_inner();
    let turn_order = turn_order.into_inner();
    for step in step_started.read() {
        if **step == turns::Step::Draw {
            let is_first_turn_skip = turn.number == 1
                && players.iter().len() == 2
                && turn_order.first() == Some(&turn.active);
            if is_first_turn_skip {
                continue;
            }
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
    fn first_player_skips_the_draw_step_of_their_first_turn() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::builder()
            .turn(1, turns::Step::Upkeep)
            .players([P1.library([GRIZZLY_BEARS, LIGHTNING_BOLT]), P2]);
        let hand_before = game.player(p1).cards_in(HAND).count();

        game.play([p1.pass_priority(), p2.pass_priority(), p1.pass_priority()])?;

        assert_eq!(game.turn().step, turns::Step::Draw);
        assert_eq!(game.player(p1).cards_in(HAND).count(), hand_before);
        Ok(())
    }

    #[test]
    fn the_active_player_draws_a_card_during_their_draw_step() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::builder()
            .turn(2, turns::Step::Upkeep)
            .players([P1.library([GRIZZLY_BEARS, LIGHTNING_BOLT]), P2]);
        let [top] = game.player(p1).cards_in(LIBRARY).exactly::<1>(GRIZZLY_BEARS)?;

        game.play([p1.pass_priority(), p2.pass_priority(), p1.pass_priority()])?;

        assert_eq!(game.zone_of(top), HAND);
        assert_eq!(
            game.player(p1).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?[0],
            top
        );
        Ok(())
    }
}
