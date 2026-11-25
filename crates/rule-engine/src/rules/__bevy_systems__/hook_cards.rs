use bevy::prelude::{Commands, MessageWriter, Query, Res, ResMut};

use crate::May;
use crate::StateBasedActionsPerformed;
use crate::actions;
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
    asked: Res<actions::AskQueue>,
) {
    let asked = asked.into_inner();
    if asked.iter().any(|ask| ask.event == E::ID) {
        return;
    }
    let Some(waiting) = &mut **queue else { return };
    while let Some(first) = waiting.first() {
        let Ok(first_owner) = owners.get(*first.card()) else {
            waiting.remove(0);
            continue;
        };
        let mut batch = Vec::new();
        while let Some(next) = waiting.first() {
            let Ok(owner) = owners.get(*next.card()) else {
                waiting.remove(0);
                continue;
            };
            if **owner != **first_owner {
                break;
            }
            batch.push(waiting.remove(0));
        }
        let mut waits_for_answer = false;
        for event in batch {
            let hooked = rules::Hooked {
                hook: event,
                player: **first_owner,
            };
            if !answers.legal(&hooked) {
                continue;
            }
            for then in answers.hooks(&hooked) {
                if matches!(then, rules::Then::May(_)) {
                    waits_for_answer = true;
                }
                then.announce(
                    &hooked,
                    &mut commands,
                    &mut asks,
                    &mut state_based_actions_performed,
                );
            }
        }
        if waits_for_answer {
            break;
        }
    }
}
