use bevy::prelude::Resource;
use derive_more::Deref;

use crate::stack;

#[derive(Resource, Debug, Default, Deref)]
pub struct TriggerQueue(pub Vec<stack::PendingTrigger>);

#[rustfmt::skip]
impl TriggerQueue {
    #[must_use]
    pub fn is_empty(&self) -> bool { (**self).is_empty() }
}

impl TriggerQueue {
    pub fn push(&mut self, trigger: stack::PendingTrigger) {
        let Self(pending) = self;
        pending.push(trigger);
    }

    pub fn take_pending(&mut self) -> Vec<stack::PendingTrigger> {
        let Self(pending) = self;
        std::mem::take(pending)
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::ANOTHER_CREATURE;
    use crate::YOU;
    use crate::abilities;
    use crate::effects;
    use crate::players;
    use crate::stack;

    use super::TriggerQueue;

    #[test]
    fn test_usage() {
        let trigger = stack::PendingTrigger {
            source: Entity::PLACEHOLDER,
            controller: players::PlayerId(Entity::PLACEHOLDER),
            ability: abilities::TriggeredAbility {
                event: ANOTHER_CREATURE.enters(),
                intervening_if: None,
                effect: effects::gain_life(1).to_player(YOU),
            },
        };
        let mut queue = TriggerQueue::default();
        assert!(queue.is_empty());
        queue.push(trigger.clone());
        assert!(!queue.is_empty());
        assert_eq!(queue.take_pending(), [trigger]);
        assert!(queue.is_empty());
    }
}
