use bevy::prelude::{MessageReader, MessageWriter, ResMut};

use crate::actions;
use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;

pub fn show_may<E: Event>(
    mut asks: MessageReader<rules::Ask<E>>,
    mut offered: MessageWriter<actions::Ask>,
    mut pending: ResMut<actions::PendingMayAsks<E>>,
    players: players::Players,
) {
    for ask in asks.read() {
        let player = ask.event.player();
        if players.is_still_in_the_game(player) {
            offered.write(actions::Ask {
                player,
                event: E::ID,
                option: ask.event.subject(),
            });
            pending.push(ask.clone());
        }
    }
    pending.retain(|ask| players.is_still_in_the_game(ask.event.player()));
}
