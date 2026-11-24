use std::collections::BTreeSet;

use bevy::prelude::{Entity, MessageReader, MessageWriter, ResMut};

use crate::actions;
use crate::players;

pub fn answer_asks(
    mut offered: MessageReader<actions::Ask>,
    mut player_actions: MessageReader<actions::PlayerAction>,
    mut answered: MessageWriter<actions::Answered>,
    mut rejected: MessageWriter<actions::Rejected>,
    mut pending: ResMut<actions::PendingChoice>,
    mut queue: ResMut<actions::AskQueue>,
    players: players::Players,
) {
    queue.extend(offered.read().cloned());
    queue.retain(|ask| players.is_still_in_the_game(ask.player));
    if let Some(actions::Choice::May(choice)) = (*pending).as_ref()
        && !players.is_still_in_the_game(choice.player)
    {
        *pending = actions::PendingChoice(None);
    }

    let show_first_group = |queue: &actions::AskQueue, pending: &mut actions::PendingChoice| {
        if pending.is_some() {
            return;
        }
        let Some(first) = queue.first() else {
            return;
        };
        let options = queue
            .iter()
            .filter(|ask| ask.player == first.player && ask.event == first.event)
            .map(|ask| ask.option)
            .collect();
        *pending = actions::PendingChoice(Some(actions::Choice::May(actions::MayChoice {
            player: first.player,
            event: first.event,
            options,
        })));
    };
    show_first_group(&queue, &mut pending);

    for action in player_actions.read() {
        let actions::Action::ChooseMay(chosen) = &action.action else {
            continue;
        };
        let Some(actions::Choice::May(choice)) = (*pending).as_ref().cloned() else {
            rejected.write(actions::Rejected::InvalidMay {
                player: action.player,
            });
            continue;
        };
        let chosen_set: BTreeSet<Entity> = chosen.iter().copied().collect();
        let options: BTreeSet<Entity> = choice.options.iter().copied().collect();
        let is_chooser = action.player == choice.player;
        let are_unique = chosen_set.len() == chosen.len();
        let are_legal = chosen_set.is_subset(&options);
        if !(is_chooser && are_unique && are_legal) {
            rejected.write(actions::Rejected::InvalidMay {
                player: action.player,
            });
            continue;
        }
        for option in &choice.options {
            answered.write(actions::Answered {
                player: choice.player,
                event: choice.event,
                option: *option,
                is_accepted: chosen_set.contains(option),
            });
        }
        queue.retain(|ask| {
            !(ask.player == choice.player
                && ask.event == choice.event
                && choice.options.contains(&ask.option))
        });
        *pending = actions::PendingChoice(None);
    }

    show_first_group(&queue, &mut pending);
}
