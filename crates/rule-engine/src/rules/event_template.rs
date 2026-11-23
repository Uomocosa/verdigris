use atomic_delegate_macros::atomic_delegates;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventTemplate {
    WinGame { who: rules::Role },
    LoseGame { who: rules::Role },
}

#[atomic_delegates]
impl EventTemplate {
    #[must_use]
    pub fn instantiate<E: Event>(&self, event: &E) -> rules::AnyEvent {}
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;

    use super::EventTemplate;

    #[test]
    fn test_usage() {
        let player = Entity::PLACEHOLDER;
        let happened = rules::DrawFromEmptyLibrary(player);
        let template = EventTemplate::WinGame {
            who: rules::Role::Player,
        };
        assert_eq!(
            template.instantiate(&happened),
            rules::AnyEvent::WinGame(rules::WinGame(player))
        );
        assert_eq!(
            EventTemplate::LoseGame {
                who: rules::Role::Subject,
            }
            .instantiate(&happened),
            rules::AnyEvent::LoseGame(rules::LoseGame(player))
        );
    }
}
