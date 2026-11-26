use bevy::prelude::{Commands, MessageWriter, Query, Res, ResMut};

use crate::May;
use crate::StateBasedActionsPerformed;
use crate::actions;
use crate::cards;
use crate::rules;
use crate::rules::traits::hook_event::HookEvent;

pub fn hook_cards<E: HookEvent>(
    mut queue: ResMut<rules::HookQueue<E>>,
    controllers: Query<&cards::Controller>,
    owners: Query<&cards::Owner>,
    mut commands: Commands,
    mut asks: MessageWriter<May<rules::Hooked<E>>>,
    mut state_based_actions_performed: ResMut<StateBasedActionsPerformed>,
    answers: rules::Rules,
    asked: Res<actions::AskQueue>,
    accepted: Res<actions::AcceptedMays<rules::Hooked<E>>>,
    choice: Res<actions::PendingChoice>,
) {
    if !accepted.into_inner().is_empty() {
        return;
    }
    let choice = choice.into_inner();
    if choice.as_ref().is_some_and(|choice| !matches!(choice, actions::Choice::May(_))) {
        return;
    }
    let asked = asked.into_inner();
    if asked.iter().any(|ask| ask.is_may_of(E::ID)) {
        return;
    }
    while let Some(events) = (**queue).as_ref() {
        if events.is_empty() {
            break;
        }
        let hooks_of_the_first_player: Vec<rules::Hooked<E>> = queue
            .take_first_group(|first, next| {
                match (
                    controllers
                        .get(*first.card())
                        .ok()
                        .map(|controller| **controller)
                        .or_else(|| owners.get(*first.card()).ok().map(|owner| **owner)),
                    controllers
                        .get(*next.card())
                        .ok()
                        .map(|controller| **controller)
                        .or_else(|| owners.get(*next.card()).ok().map(|owner| **owner)),
                ) {
                    (Some(first_player), Some(next_player)) => first_player == next_player,
                    _ => false,
                }
            })
            .into_iter()
            .filter_map(|hook| {
                let player = controllers
                    .get(*hook.card())
                    .ok()
                    .map(|controller| **controller)
                    .or_else(|| owners.get(*hook.card()).ok().map(|owner| **owner))?;
                let hooked = rules::Hooked { hook, player };
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
        if thens_of_the_first_player.is_empty() {
            continue;
        }
        for (hooked, then) in thens_of_the_first_player {
            then.announce(
                &hooked,
                &mut commands,
                &mut asks,
                &mut state_based_actions_performed,
            );
        }
        break;
    }
}
