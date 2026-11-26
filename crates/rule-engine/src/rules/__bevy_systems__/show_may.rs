use bevy::prelude::{MessageReader, MessageWriter, ResMut};

use crate::May;
use crate::actions;
use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;

pub fn show_may<E: Event>(
    mut asks: MessageReader<May<E>>,
    mut offered: MessageWriter<actions::Ask>,
    mut pending: ResMut<actions::PendingMayAsks<E>>,
    players: players::Players,
) {
    for ask in asks.read() {
        let Some(player) = E::args().iter().find_map(|id| match ask.event.arg(*id) {
            Some(rules::ArgValue::Player(player)) => Some(player),
            _ => None,
        }) else {
            continue;
        };
        if players.is_still_in_the_game(player) {
            offered.write(actions::Ask(actions::Choice::May(actions::MayChoice {
                player,
                event: E::ID,
                options: E::args()
                    .iter()
                    .filter_map(|id| match ask.event.arg(*id) {
                        Some(rules::ArgValue::Entity(entity)) => Some(entity),
                        _ => None,
                    })
                    .collect(),
            })));
            pending.push(ask.clone());
        }
    }
    pending.retain(|ask| {
        E::args()
            .iter()
            .find_map(|id| match ask.event.arg(*id) {
                Some(rules::ArgValue::Player(player)) => Some(player),
                _ => None,
            })
            .is_some_and(|player| players.is_still_in_the_game(player))
    });
}
