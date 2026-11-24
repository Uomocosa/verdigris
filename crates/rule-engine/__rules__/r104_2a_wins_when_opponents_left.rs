use crate::events;
use bevy::prelude::MessageWriter;

use crate::Resolve;
use crate::players;

/// 104.2a A player still in the game wins the game if that player’s opponents have all left the game. This
/// happens immediately and overrides all effects that would preclude that player from winning the game.
pub fn r104_2a_wins_when_opponents_left(
    mut wins: MessageWriter<Resolve<events::WinGame>>,
    players: players::Players,
) {
    for player in players.still_in_the_game() {
        if player.have_all_opponents_left() {
            wins.write(Resolve(events::WinGame(player.id)));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::actions;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn r104_2a_sole_survivor_wins_despite_cant_win_effect() {
        let (mut game, [p1, p2]) = MockGame::new([P1.battlefield([PLATINUM_ANGEL]), P2]);

        game.act(p1, actions::Action::Concede);

        assert!(game.has_lost(p1));
        assert!(game.has_won(p2));
    }

    #[test]
    fn last_player_wins_when_their_opponent_has_left() -> TestResult {
        let (mut game, [p1, _p2]) = MockGame::new([P1, P2.life(0)]);
        game.play([p1.pass_priority()])?;
        assert!(game.has_won(p1));
        Ok(())
    }

    #[test]
    fn last_player_wins_when_every_opponent_has_left() -> TestResult {
        let (mut game, [p1, p2, p3]) = MockGame::new([P1, P2.life(0), P3.life(0)]);

        game.play([p1.pass_priority()])?;

        assert!(game.has_lost(p2));
        assert!(game.has_lost(p3));
        assert!(game.has_won(p1));
        assert!(!game.has_won(p2));
        assert!(!game.has_won(p3));
        Ok(())
    }

    #[test]
    fn no_winner_is_declared_while_an_opponent_is_still_in_the_game() -> TestResult {
        let (mut game, [p1, p2, p3]) = MockGame::new([P1, P2.life(0), P3]);

        game.play([p1.pass_priority()])?;

        assert!(game.has_lost(p2));
        assert!(!game.has_won(p1));
        assert!(!game.has_won(p3));
        Ok(())
    }

    #[test]
    fn conceding_after_winning_does_not_make_the_opponent_win() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::builder()
            .turn(2, turns::Step::Upkeep)
            .players([P1.battlefield([LABORATORY_MANIAC]).empty_library(), P2]);
        game.play([p1.pass_priority(), p2.pass_priority(), p1.pass_priority()])?;
        game.act(p1, actions::Action::Concede);
        assert!(game.has_won(p1));
        assert!(!game.has_won(p2));
        Ok(())
    }
}
