use bevy::prelude::{Messages, ResMut};

use crate::actions;
use crate::players;
use crate::turns;

pub fn show_asks(
    mut asks: ResMut<Messages<actions::Ask>>,
    mut pending: ResMut<actions::PendingChoice>,
    mut queue: ResMut<actions::AskQueue>,
    mut state: ResMut<turns::TurnState>,
    players: players::Players,
) {
    queue.ingest(&mut asks, &players);
    pending.refresh(&queue, &players);
    if pending.blocks_priority() && state.suspended_priority_holder.is_none() {
        state.suspended_priority_holder = state.priority.holder.take();
    }
}
