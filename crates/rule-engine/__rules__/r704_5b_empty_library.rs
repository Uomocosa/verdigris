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
    use crate::actions;
    use crate::mana;
    use crate::players;
    use crate::testing::*;
    use crate::zones;

    #[test]
    fn test_usage() {
        let mut game = MockGame::builder()
            .hand(Seat::P1, [divination()])
            .library(Seat::P1, [])
            .mana_pool(Seat::P1, mana![2, U])
            .build();
        let divination = game.find(game.p1, zones::Zone::Hand, "Divination").unwrap();
        game.act(game.p1, actions::Action::Cast(divination));
        game.pass_priority_all();
        assert!(game.app.world().get::<players::Lost>(game.p1).is_some());
    }

    #[test]
    fn drawing_from_an_empty_library_loses_the_game_at_the_next_state_based_check() {
        let mut game = MockGame::builder()
            .hand(Seat::P1, [divination()])
            .library(Seat::P1, [])
            .mana_pool(Seat::P1, mana![2, U])
            .build();
        let divination = game.find(game.p1, zones::Zone::Hand, "Divination").unwrap();
        game.act(game.p1, actions::Action::Cast(divination));
        assert!(game.app.world().get::<players::Lost>(game.p1).is_none());
        game.pass_priority_all();
        assert!(game.app.world().get::<players::Lost>(game.p1).is_some());
    }
}
