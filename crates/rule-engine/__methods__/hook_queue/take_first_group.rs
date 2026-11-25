use crate::rules;
use crate::rules::traits::hook_event::HookEvent;

pub fn take_first_group<E: HookEvent>(
    queue: &mut rules::HookQueue<E>,
    same_group: impl Fn(&E, &E) -> bool,
) -> Vec<E> {
    let Some(events) = &mut **queue else {
        return Vec::new();
    };
    let end = events.first().map_or(0, |first| {
        events.iter().position(|next| !same_group(first, next)).unwrap_or(events.len())
    });
    events.drain(..end).collect()
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::cards;
    use crate::events;
    use crate::rules;
    use crate::rules::traits::hook_event::HookEvent;

    use super::take_first_group;

    #[test]
    fn test_usage() {
        let first = cards::CardId(Entity::PLACEHOLDER);
        let second = cards::CardId(Entity::from_raw_u32(1).unwrap());
        let mut queue = rules::HookQueue::<events::OpeningHandAction>::default();
        queue.announce(vec![
            events::OpeningHandAction(first),
            events::OpeningHandAction(first),
            events::OpeningHandAction(second),
        ]);
        let group = take_first_group(&mut queue, |current, next| current.card() == next.card());
        assert_eq!(group.len(), 2);
        assert!(group.iter().all(|event| event.card() == first));
        assert_eq!(queue.as_ref().map(Vec::len), Some(1));
    }
}
