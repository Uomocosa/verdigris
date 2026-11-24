use bevy::prelude::{MessageReader, MessageWriter};

use crate::Resolve;
use crate::actions;
use crate::players;
use crate::rules;

/// 104.3a A player can concede the game at any time. A player who concedes leaves the game
/// immediately. That player loses the game.
pub fn r104_3a_concede(
    mut actions: MessageReader<actions::PlayerAction>,
    mut losses: MessageWriter<Resolve<rules::LoseGame>>,
    players: players::Players,
) {
    for action in actions.read() {
        if action.action == actions::Action::Concede && players.is_still_in_the_game(action.player)
        {
            losses.write(Resolve(rules::LoseGame(action.player)));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::actions;
    use crate::testing::*;

    #[test]
    fn conceding_leaves_the_game_and_the_opponent_wins() {
        let (mut game, [p1, p2]) = MockGame::new([P1, P2]);

        game.act(p1, actions::Action::Concede);

        assert!(game.has_lost(p1));
        assert!(game.has_won(p2));
    }

    #[test]
    fn platinum_angel_controller_can_still_concede() {
        let (mut game, [p1, _p2]) = MockGame::new([P1.battlefield([PLATINUM_ANGEL]), P2]);

        game.act(p1, actions::Action::Concede);

        assert!(game.has_lost(p1));
    }
}
