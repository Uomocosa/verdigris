use bevy::prelude::{Local, MessageReader, MessageWriter, Res};

use crate::actions;
use crate::rules;
use crate::rules::traits::event::Event;
use crate::turns;

pub fn opening_hand_actions_complete(
    mut mulligans_complete: MessageReader<turns::MulligansComplete>,
    mut complete: MessageWriter<turns::OpeningHandActionsComplete>,
    queue: Res<actions::AskQueue>,
    mut started: Local<bool>,
    mut written: Local<bool>,
) {
    if mulligans_complete.read().count() > 0 {
        *started = true;
    }
    if !*started || *written {
        return;
    }
    let queue = queue.into_inner();
    if queue.iter().any(|ask| ask.event == rules::OpeningHandAction::ID) {
        return;
    }
    complete.write(turns::OpeningHandActionsComplete);
    *written = true;
}
