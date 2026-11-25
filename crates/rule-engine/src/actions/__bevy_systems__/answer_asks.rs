use bevy::prelude::{MessageReader, MessageWriter, Messages, ResMut};

use crate::actions;
use crate::players;

pub fn answer_asks(
    mut asks: ResMut<Messages<actions::Ask>>,
    mut player_actions: MessageReader<actions::PlayerAction>,
    mut answered: MessageWriter<actions::Answered>,
    mut rejected: MessageWriter<actions::Rejected>,
    mut pending: ResMut<actions::PendingChoice>,
    mut queue: ResMut<actions::AskQueue>,
    players: players::Players,
) {
    queue.ingest(&mut asks, &players);
    pending.refresh(&queue, &players);
    for action in player_actions.read() {
        let Some(rejection) = action.action.rejection(action.player) else {
            continue;
        };
        let Some(choice) = (*pending).as_ref().cloned() else {
            rejected.write(rejection);
            continue;
        };
        let is_chooser = choice.player().is_none_or(|player| player == action.player);
        if !(is_chooser && choice.accepts(&action.action)) {
            rejected.write(rejection);
            continue;
        }
        answered.write(actions::Answered {
            choice: choice.clone(),
            answer: action.action.clone(),
        });
        let mut has_removed_first_equal = false;
        queue.retain(|ask| {
            let asked: &actions::Choice = ask;
            match (asked, &choice) {
                (actions::Choice::May(asked), actions::Choice::May(answered)) => {
                    asked.player != answered.player || asked.event != answered.event
                }
                (asked, answered) => {
                    if !has_removed_first_equal && asked == answered {
                        has_removed_first_equal = true;
                        false
                    } else {
                        true
                    }
                }
            }
        });
        *pending = actions::PendingChoice(None);
    }
    pending.refresh(&queue, &players);
}
