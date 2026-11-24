use bevy::prelude::Commands;

use crate::Game;
use crate::GameEnded;

/// 104.1. A game ends immediately when a player wins, when the game is a draw, or when the game is restarted.
pub fn r104_1_game_ends(mut commands: Commands, game: Game) {
    if game.is_over() {
        commands.insert_resource(GameEnded);
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::actions;
    use crate::players;
    use crate::testing::*;

    #[test]
    fn nothing_happens_after_a_player_has_won() {
        let (mut game, [p1, _p2]) = MockGame::new([P1, P2.life(0)]);
        game.act(p1, actions::Action::PassPriority);
        assert!(game.has_won(p1));
        let priority = game.priority();
        let turn = *game.turn();
        let holder: Entity = *game.priority().holder.unwrap_or(p1.id);
        game.act(holder, actions::Action::PassPriority);
        assert_eq!(game.priority(), priority);
        assert_eq!(*game.turn(), turn);
    }

    #[test]
    fn nothing_happens_after_a_draw() {
        let (mut game, [p1, _p2]) = MockGame::new([P1.life(0), P2.life(0)]);
        game.act(p1, actions::Action::PassPriority);
        assert!(game.has::<players::GameIsADraw>(p1));
        let priority = game.priority();
        let turn = *game.turn();
        let holder: Entity = *game.priority().holder.unwrap_or(p1.id);
        game.act(holder, actions::Action::PassPriority);
        assert_eq!(game.priority(), priority);
        assert_eq!(*game.turn(), turn);
    }
}
