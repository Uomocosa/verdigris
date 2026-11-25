use std::marker::PhantomData;

use bevy::prelude::{MessageWriter, Res, ResMut};

use crate::HooksDone;
use crate::actions;
use crate::rules;
use crate::rules::traits::hook_event::HookEvent;

pub fn hooks_done<E: HookEvent>(
    mut queue: ResMut<rules::HookQueue<E>>,
    asks: Res<actions::AskQueue>,
    accepted: Res<actions::AcceptedMays<rules::Hooked<E>>>,
    mut done: MessageWriter<HooksDone<E>>,
) {
    let asks = asks.into_inner();
    let accepted = accepted.into_inner();
    let Some(events) = (**queue).as_ref() else {
        return;
    };
    if !events.is_empty() || asks.iter().any(|ask| ask.is_may_of(E::ID)) || !accepted.is_empty() {
        return;
    }
    done.write(HooksDone(PhantomData));
    **queue = None;
}
