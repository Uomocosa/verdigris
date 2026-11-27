use std::collections::BTreeSet;

use bevy::prelude::Entity;

use crate::actions;

pub fn first_queued(queue: &actions::AskQueue) -> Option<actions::Choice> {
    let first: &actions::Choice = queue.first()?;
    let actions::Choice::Pick(first) = first else {
        return Some(first.clone());
    };
    if first.count.min != 0 {
        return Some(actions::Choice::Pick(first.clone()));
    }
    let options: BTreeSet<Entity> = queue
        .iter()
        .filter_map(|ask| {
            let asked: &actions::Choice = ask;
            let actions::Choice::Pick(next) = asked else {
                return None;
            };
            (next.player == first.player && next.id == first.id && next.count.min == 0)
                .then(|| next.among.clone())
        })
        .flatten()
        .collect();
    let count = actions::Count::up_to(u32::try_from(options.len()).unwrap_or(u32::MAX));
    Some(actions::Choice::Pick(actions::PickChoice {
        player: first.player,
        id: first.id,
        source: None,
        among: options,
        count,
        ordered: false,
    }))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use bevy::prelude::Entity;

    use crate::BATTLEFIELD;
    use crate::HAND;
    use crate::actions;
    use crate::testing::*;

    use super::first_queued;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::new([P1, P2]);
        let first = game.zone_entity(p1, BATTLEFIELD).unwrap();
        let queue = actions::AskQueue(vec![actions::Ask(actions::Choice::Pick(
            actions::PickChoice {
                id: actions::ChoiceId("may"),
                player: p1.id,
                source: None,
                among: BTreeSet::from([first]),
                count: actions::Count::up_to(1),
                ordered: false,
            },
        ))]);
        assert_eq!(
            first_queued(&queue),
            Some(actions::Choice::Pick(actions::PickChoice {
                id: actions::ChoiceId("may"),
                player: p1.id,
                source: None,
                among: BTreeSet::from([first]),
                count: actions::Count::up_to(1),
                ordered: false,
            }))
        );
    }

    #[test]
    fn consecutive_may_picks_of_one_player_and_event_group_in_queue_order() {
        let (game, [p1, p2, ..]) = MockGame::new([P1, P2, P3]);
        let first = game.zone_entity(p1, BATTLEFIELD).unwrap();
        let second = game.zone_entity(p1, HAND).unwrap();
        let other = game.zone_entity(p2, BATTLEFIELD).unwrap();
        let event = actions::ChoiceId("may");
        let may = |options: BTreeSet<Entity>| {
            actions::Ask(actions::Choice::Pick(actions::PickChoice {
                id: event,
                player: p1.id,
                source: None,
                among: options,
                count: actions::Count::up_to(1),
                ordered: false,
            }))
        };
        let queue = actions::AskQueue(vec![
            may(BTreeSet::from([first])),
            may(BTreeSet::from([second])),
            actions::Ask(actions::Choice::Pick(actions::PickChoice {
                id: event,
                player: p2.id,
                source: None,
                among: BTreeSet::from([other]),
                count: actions::Count::up_to(1),
                ordered: false,
            })),
            actions::Ask(actions::Choice::Pick(actions::PickChoice {
                id: actions::ChoiceId("701.9b/discard"),
                player: p1.id,
                source: None,
                among: BTreeSet::from([first]),
                count: actions::Count::exactly(1),
                ordered: false,
            })),
        ]);
        assert_eq!(
            first_queued(&queue),
            Some(actions::Choice::Pick(actions::PickChoice {
                id: event,
                player: p1.id,
                source: None,
                among: BTreeSet::from([first, second]),
                count: actions::Count::up_to(2),
                ordered: false,
            }))
        );
        assert_eq!(first_queued(&actions::AskQueue::default()), None);
    }
}
