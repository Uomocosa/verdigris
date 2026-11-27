use bevy::prelude::{Entity, Resource};
use derive_more::Deref;

use crate::abilities;
use crate::events;
use crate::rules;
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

    pub(crate) fn push_checked(
        &mut self,
        source: Entity,
        ability: abilities::TriggeredAbility,
        answers: &rules::Rules,
    ) {
        let triggers = ability.intervening_if.as_ref().is_none_or(|condition| {
            answers.eval(
                condition,
                &events::AbilityContext {
                    ability: source,
                    source,
                },
                None,
            )
        });
        if triggers {
            self.push(stack::PendingTrigger { source, ability });
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::YOU;
    use crate::abilities;
    use crate::another_creature;
    use crate::stack;

    use super::TriggerQueue;

    #[test]
    fn test_usage() {
        let trigger = stack::PendingTrigger {
            source: Entity::PLACEHOLDER,
            ability: abilities::TriggeredAbility {
                event: another_creature().enters(),
                intervening_if: None,
                effect: YOU.gain_life(1),
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
