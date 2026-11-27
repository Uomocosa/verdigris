use bevy::prelude::{Commands, Entity, MessageReader, MessageWriter, Res, ResMut};

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
        if !answer.choice.is_may_of(E::ID) {
            continue;
        }
        let actions::Choice::Pick(choice) = &answer.choice else {
            continue;
        };
        let actions::Action::Choose(chosen) = &answer.answer else {
            continue;
        };
        let decisions: Vec<(&Entity, bool)> = chosen
            .iter()
            .map(|option| (option, true))
            .chain(
                choice
                    .fixed()
                    .into_iter()
                    .flatten()
                    .filter(|option| !chosen.contains(option))
                    .map(|option| (option, false)),
            )
            .collect();
        for (option, is_accepted) in decisions {
            let Some(index) = pending.iter().position(|ask| {
                E::args()
                    .iter()
                    .find_map(|id| match ask.event.arg(*id) {
                        Some(rules::ArgValue::Player(player)) => Some(player),
                        _ => None,
                    })
                    .is_some_and(|player| player == choice.player)
                    && E::args()
                        .iter()
                        .any(|id| ask.event.arg(*id) == Some(rules::ArgValue::Entity(*option)))
            }) else {
                continue;
            };
            let ask = pending.remove(index);
            if is_accepted {
                accepted.push(ask);
            } else {
                proceeds.write(Resolve(ask.event));
            }
        }
    }
    let choice = choice.into_inner();
    let announce_is_open = choice.as_ref().is_none_or(
        |choice| matches!(choice, actions::Choice::Pick(choice) if choice.count.min == 0),
    );
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
        ask.then.instantiate(&ask.event, &answers).announce(&mut commands);
        break;
    }
}
