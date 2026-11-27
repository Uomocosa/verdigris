use atomic_delegate_macros::atomic_delegates;
use bevy::ecs::schedule::{IntoScheduleConfigs, ScheduleConfigs};
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::Message;
use derive_more::Deref;

use crate::rules::traits::event::Event;

#[derive(Message, Clone, Debug, Deref)]
pub struct Attempt<E>(pub E);

#[atomic_delegates]
impl<E: Event> Attempt<E> {
    pub fn resolved_by<ResolvedEvent, Marker>(
        default: impl IntoScheduleConfigs<ScheduleSystem, Marker>,
    ) -> ScheduleConfigs<ScheduleSystem>
    where
        Self: ::std::ops::Deref<Target = ResolvedEvent>,
        ResolvedEvent: Event,
    {
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::events;
    use crate::rules::traits::event::Event;

    use super::Attempt;

    #[test]
    fn test_usage() {
        let untap = events::Untap {
            permanent: Entity::PLACEHOLDER,
            cause: events::Cast::ID,
        };
        let attempt = Attempt(untap);
        assert_eq!(attempt.permanent, Entity::PLACEHOLDER);
    }
}
