use bevy::prelude::{Commands, Entity, Query, With, Without};

use crate::players;

/// 704.5b If a player attempted to draw a card from a library with no cards in it since the last
/// time state-based actions were checked, that player loses the game.
pub fn r704_5b_empty_library(
    mut commands: Commands,
    players: Query<
        Entity,
        (
            With<players::AttemptedDrawFromEmptyLibrary>,
            Without<players::Lost>,
        ),
    >,
) {
    for player in players.iter() {
        commands.entity(player).insert(players::Lost);
    }
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::actions;
    use crate::mana;
    use crate::players;
    use crate::testing::*;

    #[test]
    fn test_usage() {
        let (mut game, [p1, ..]) =
            MockGame::new([P1.hand([DIVINATION]).library([]).mana_pool(mana![2, U]), P2]);
        let divination = game.player(p1).cards(HAND, DIVINATION).only().unwrap();
        game.act(p1, actions::Action::Cast(divination));
        game.pass_priority_all();
        assert!(game.app.world().get::<players::Lost>(*p1).is_some());
    }

    #[test]
    fn drawing_from_an_empty_library_loses_the_game_at_the_next_state_based_check() {
        let (mut game, [p1, ..]) =
            MockGame::new([P1.hand([DIVINATION]).library([]).mana_pool(mana![2, U]), P2]);
        let divination = game.player(p1).cards(HAND, DIVINATION).only().unwrap();
        game.act(p1, actions::Action::Cast(divination));
        assert!(game.app.world().get::<players::Lost>(*p1).is_none());
        game.pass_priority_all();
        assert!(game.app.world().get::<players::Lost>(*p1).is_some());
    }
}
