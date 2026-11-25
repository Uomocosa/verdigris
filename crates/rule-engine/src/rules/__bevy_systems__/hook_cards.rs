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
    accepted: Res<actions::AcceptedMays<rules::Hooked<E>>>,
) {
    if !accepted.into_inner().is_empty() {
        return;
    }
    let asked = asked.into_inner();
    if asked.iter().any(|ask| ask.event == E::ID) {
        return;
    }
    while let Some(events) = (**queue).as_ref() {
        if events.is_empty() {
            break;
        }
        let hooks_of_the_first_player: Vec<rules::Hooked<E>> = queue
            .take_first_group(|first, next| {
                match (owners.get(*first.card()), owners.get(*next.card())) {
                    (Ok(first_owner), Ok(next_owner)) => **first_owner == **next_owner,
                    _ => false,
                }
            })
            .into_iter()
            .filter_map(|hook| {
                let owner = owners.get(*hook.card()).ok()?;
                let hooked = rules::Hooked {
                    hook,
                    player: **owner,
                };
                answers.legal(&hooked).then_some(hooked)
            })
            .collect();
        if hooks_of_the_first_player.is_empty() {
            continue;
        }
        let thens_of_the_first_player: Vec<(rules::Hooked<E>, rules::Then)> =
            hooks_of_the_first_player
                .iter()
                .flat_map(|hooked| {
                    answers.hooks(hooked).into_iter().map(|then| (hooked.clone(), then))
                })
                .collect();
        let waits_for_answer =
            thens_of_the_first_player.iter().any(|(_, then)| matches!(then, rules::Then::May(_)));
        for (hooked, then) in thens_of_the_first_player {
            then.announce(
                &hooked,
                &mut commands,
                &mut asks,
                &mut state_based_actions_performed,
            );
        }
        if waits_for_answer {
            break;
        }
    }
}
