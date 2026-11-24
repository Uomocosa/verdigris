use bevy::prelude::{Commands, MessageReader, MessageWriter, ResMut};

use crate::actions;
use crate::rules;
use crate::rules::traits::event::Event;

pub fn answer_may<E: Event>(
    mut answered: MessageReader<actions::Answered>,
    mut proceeds: MessageWriter<rules::Proceeds<E>>,
    mut commands: Commands,
    mut pending: ResMut<actions::PendingMayAsks<E>>,
) {
    for answer in answered.read() {
        if answer.event != E::ID {
            continue;
        }
        let Some(index) = pending.iter().position(|ask| {
            ask.event.player() == answer.player && ask.event.subject() == answer.option
        }) else {
            continue;
        };
        let ask = pending.remove(index);
        if answer.is_accepted {
            ask.then.instantiate(&ask.event).announce(&mut commands);
        } else {
            proceeds.write(rules::Proceeds(ask.event));
        }
    }
}
