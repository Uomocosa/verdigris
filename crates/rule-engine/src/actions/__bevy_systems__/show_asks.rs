use bevy::prelude::{Messages, ResMut};

use crate::actions;
use crate::players;

pub fn show_asks(
    mut asks: ResMut<Messages<actions::Ask>>,
    mut pending: ResMut<actions::PendingChoice>,
    mut queue: ResMut<actions::AskQueue>,
    players: players::Players,
) {
    queue.ingest(&mut asks, &players);
    pending.refresh(&queue, &players);
}
