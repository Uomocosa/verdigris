use bevy::prelude::Resource;
use derive_more::{Deref, DerefMut};

use crate::rules::traits::hook_event::HookEvent;

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct HookQueue<E: HookEvent>(Option<Vec<E>>);

impl<E: HookEvent> Default for HookQueue<E> {
    fn default() -> Self {
        Self(None)
    }
}

#[rustfmt::skip]
impl<E: HookEvent> HookQueue<E> {
    pub fn announce(&mut self, events: Vec<E>) { **self = Some(events); }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::cards;
    use crate::events;

    use super::HookQueue;

    #[test]
    fn test_usage() {
        let card = cards::CardId(Entity::PLACEHOLDER);
        let mut queue = HookQueue::<events::OpeningHandAction>::default();
        assert!(queue.is_none());
        queue.announce(vec![events::OpeningHandAction(card)]);
        assert_eq!(queue.as_ref().map(Vec::len), Some(1));
    }
}
