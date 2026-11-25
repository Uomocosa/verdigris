use bevy::prelude::{Commands, MessageReader, MessageWriter, Res, ResMut};

use crate::Resolve;
use crate::actions;
use crate::rules;
use crate::rules::traits::event::Event;

pub fn answer_may<E: Event>(
    mut answered: MessageReader<actions::Answered>,
    mut proceeds: MessageWriter<Resolve<E>>,
    mut commands: Commands,
    mut pending: ResMut<actions::PendingMayAsks<E>>,
    mut accepted: ResMut<actions::AcceptedMays<E>>,
    choice: Res<actions::PendingChoice>,
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
            accepted.push(ask);
        } else {
            proceeds.write(Resolve(ask.event));
        }
    }
    let choice = choice.into_inner();
    let announce_is_open =
        choice.as_ref().is_none_or(|choice| matches!(choice, actions::Choice::May(_)));
    loop {
        let Some(ask) = accepted.first().cloned() else {
            break;
        };
        if !answers.legal(&ask.event) {
            accepted.remove(0);
            continue;
        }
        if !announce_is_open {
            break;
        }
        accepted.remove(0);
        ask.then.instantiate(&ask.event).announce(&mut commands);
        break;
    }
}
