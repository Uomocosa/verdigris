use bevy::prelude::Entity;

use crate::actions;

pub fn first_queued(queue: &actions::AskQueue) -> Option<actions::Choice> {
    let first: &actions::Choice = queue.first()?;
    let actions::Choice::May(first) = first else {
        return Some(first.clone());
    };
    let options: Vec<Entity> = queue
        .iter()
        .filter_map(|ask| {
            let asked: &actions::Choice = ask;
            let actions::Choice::May(next) = asked else {
                return None;
            };
            (next.player == first.player && next.event == first.event).then(|| next.options.clone())
        })
        .flatten()
        .collect();
    Some(actions::Choice::May(actions::MayChoice {
        player: first.player,
        event: first.event,
        options,
    }))
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::HAND;
    use crate::actions;
    use crate::rules;
    use crate::testing::*;

    use super::first_queued;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::new([P1, P2]);
        let first = game.zone_entity(p1, BATTLEFIELD).unwrap();
        let queue = actions::AskQueue(vec![actions::Ask(actions::Choice::May(
            actions::MayChoice {
                player: p1.id,
                event: rules::EventId("may"),
                options: vec![first],
            },
        ))]);
        assert_eq!(
            first_queued(&queue),
            Some(actions::Choice::May(actions::MayChoice {
                player: p1.id,
                event: rules::EventId("may"),
                options: vec![first],
            }))
        );
    }

    #[test]
    fn consecutive_mays_of_one_player_and_event_group_in_queue_order() {
        let (game, [p1, p2, ..]) = MockGame::new([P1, P2, P3]);
        let first = game.zone_entity(p1, BATTLEFIELD).unwrap();
        let second = game.zone_entity(p1, HAND).unwrap();
        let other = game.zone_entity(p2, BATTLEFIELD).unwrap();
        let event = rules::EventId("may");
        let queue = actions::AskQueue(vec![
            actions::Ask(actions::Choice::May(actions::MayChoice {
                player: p1.id,
                event,
                options: vec![first],
            })),
            actions::Ask(actions::Choice::May(actions::MayChoice {
                player: p1.id,
                event,
                options: vec![second],
            })),
            actions::Ask(actions::Choice::May(actions::MayChoice {
                player: p2.id,
                event,
                options: vec![other],
            })),
        ]);
        assert_eq!(
            first_queued(&queue),
            Some(actions::Choice::May(actions::MayChoice {
                player: p1.id,
                event,
                options: vec![first, second],
            }))
        );
        assert_eq!(first_queued(&actions::AskQueue::default()), None);
    }
}
