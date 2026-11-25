use bevy::prelude::{Commands, MessageWriter, Query, ResMut};

use crate::May;
use crate::StateBasedActionsPerformed;
use crate::cards;
use crate::rules;
use crate::rules::traits::hook_event::HookEvent;

pub fn hook_cards<E: HookEvent>(
    mut queue: ResMut<rules::HookQueue<E>>,
    owners: Query<&cards::Owner>,
    mut commands: Commands,
    mut asks: MessageWriter<May<rules::Hooked<E>>>,
    mut state_based_actions_performed: ResMut<StateBasedActionsPerformed>,
    answers: rules::Rules,
) {
    let Some(waiting) = &mut **queue else { return };
    let events = std::mem::take(waiting);
    for event in events {
        let Ok(owner) = owners.get(*event.card()) else {
            continue;
        };
        let hooked = rules::Hooked {
            hook: event,
            player: **owner,
        };
        if !answers.legal(&hooked) {
            continue;
        }
        for then in answers.hooks(&hooked) {
            then.announce(
                &hooked,
                &mut commands,
                &mut asks,
                &mut state_based_actions_performed,
            );
        }
    }
}
