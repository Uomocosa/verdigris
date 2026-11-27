use bevy::prelude::{MessageReader, MessageWriter, Messages, ParamSet, ResMut};

use crate::actions;
use crate::players;
use crate::rules;
use crate::turns;

pub fn answer_asks(
    mut asks: ResMut<Messages<actions::Ask>>,
    mut player_actions: MessageReader<actions::PlayerAction>,
    mut answered: MessageWriter<actions::Answered>,
    mut rejected: MessageWriter<actions::Rejected>,
    mut pending: ResMut<actions::PendingChoice>,
    mut queue: ResMut<actions::AskQueue>,
    mut state_and_answers: ParamSet<(ResMut<turns::TurnState>, rules::Rules)>,
    players: players::Players,
) {
    queue.ingest(&mut asks, &players);
    pending.refresh(&queue, &players);
    for action in player_actions.read() {
        if !action.action.answers_a_choice() {
            continue;
        }
        let Some(choice) = (*pending).as_ref().cloned() else {
            if let Some(rejection) = action.action.rejection(action.player) {
                rejected.write(rejection);
            }
            continue;
        };
        let is_chooser = choice.player() == action.player;
        let accepted = {
            let answers = state_and_answers.p1();
            is_chooser && choice.accepts(&answers, &action.action)
        };
        if !accepted {
            let rejection = {
                let answers = state_and_answers.p1();
                choice.rejection(&answers, action.player, &action.action)
            };
            if let Some(rejection) = rejection {
                rejected.write(rejection);
            }
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
                (actions::Choice::Pick(asked), actions::Choice::Pick(answered))
                    if asked.player == answered.player
                        && asked.id == answered.id
                        && asked.count.min == 0
                        && answered.count.min == 0 =>
                {
                    false
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
    if !pending.blocks_priority() {
        state_and_answers.p0().resume_priority();
    }
}
