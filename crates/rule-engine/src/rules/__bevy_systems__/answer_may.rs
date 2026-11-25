use bevy::prelude::{Commands, MessageReader, MessageWriter, ResMut};

use crate::Resolve;
use crate::actions;
use crate::rules;
use crate::rules::traits::event::Event;

pub fn answer_may<E: Event>(
    mut answered: MessageReader<actions::Answered>,
    mut proceeds: MessageWriter<Resolve<E>>,
    mut commands: Commands,
    mut pending: ResMut<actions::PendingMayAsks<E>>,
    answers: rules::Rules,
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
            if answers.legal(&ask.event) {
                ask.then.instantiate(&ask.event).announce(&mut commands);
            }
        } else {
            proceeds.write(Resolve(ask.event));
        }
    }
}
