use crate::rules;
use crate::rules::traits::event::Event;

pub fn instantiate<E: Event>(template: &rules::EventTemplate, event: &E) -> rules::AnyEvent {
    let holder = |who: &rules::Role| match who {
        rules::Role::Subject => event.subject(),
        rules::Role::Other => event.other().unwrap_or_else(|| event.player()),
        rules::Role::Player => event.player(),
    };
    match template {
        rules::EventTemplate::WinGame { who } => {
            rules::AnyEvent::WinGame(rules::WinGame(holder(who)))
        }
        rules::EventTemplate::LoseGame { who } => {
            rules::AnyEvent::LoseGame(rules::LoseGame(holder(who)))
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;

    use super::instantiate;

    #[test]
    fn test_usage() {
        let player = Entity::PLACEHOLDER;
        let happened = rules::DrawFromEmptyLibrary(player);
        assert_eq!(
            instantiate(
                &rules::EventTemplate::WinGame {
                    who: rules::Role::Player,
                },
                &happened
            ),
            rules::AnyEvent::WinGame(rules::WinGame(player))
        );
    }
}
