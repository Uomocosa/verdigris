use bevy::prelude::MessageMutator;

use crate::Game;
use crate::players;
use crate::turns;

/// 103.8a In a two-player game, the player who plays first skips the draw step (see rule 504, “Draw
/// Step”) of their first turn.
pub fn r103_8a_skip_first_draw_step(
    mut next_step: MessageMutator<turns::NextStep>,
    game: Game,
    players: players::Players,
) {
    for step in next_step.read() {
        if **step == turns::Step::Draw && game.is_first_turn() && players.is_two_player_game() {
            *step = turns::NextStep(turns::Step::PrecombatMain);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn first_player_skips_the_draw_step_of_their_first_turn() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::builder()
            .turn(1, turns::Step::Upkeep)
            .players([P1.library([GRIZZLY_BEARS, LIGHTNING_BOLT]), P2]);
        let hand_before = game.player(p1).cards_in(HAND).count();

        game.play([p1.pass_priority(), p2.pass_priority()])?;

        assert_eq!(game.turn().step, turns::Step::PrecombatMain);
        assert_eq!(game.player(p1).cards_in(HAND).count(), hand_before);
        Ok(())
    }
}
