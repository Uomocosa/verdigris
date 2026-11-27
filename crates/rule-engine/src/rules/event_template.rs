use atomic_delegate_macros::atomic_delegates;

use crate::counters;
use crate::rules;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventTemplate {
    WinGame {
        who: rules::ArgRef,
    },
    LoseGame {
        who: rules::ArgRef,
    },
    Draw {
        who: rules::ArgRef,
        amount: rules::Amount,
    },
    LoseLife {
        who: rules::ArgRef,
        amount: rules::Amount,
    },
    Discard {
        who: rules::ArgRef,
        amount: rules::Amount,
    },
    GainLife {
        who: rules::ArgRef,
        amount: rules::Amount,
    },
    Mill {
        who: rules::ArgRef,
        amount: rules::Amount,
    },
    PutCounters {
        who: rules::ArgRef,
        kind: counters::CounterId,
        amount: rules::Amount,
    },
    Exile {
        what: rules::ArgRef,
    },
    BeginTheGameOnTheBattlefield,
}

#[atomic_delegates]
impl EventTemplate {
    #[must_use]
    pub fn instantiate<E: rules::traits::event::Event>(
        &self,
        event: &E,
        answers: &rules::Rules,
    ) -> rules::AnyEvent {
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::events;
    use crate::players;
    use crate::rules;

    use super::EventTemplate;

    #[test]
    fn test_usage() {
        use bevy::ecs::system::RunSystemOnce;

        use crate::testing::MockGame;
        use crate::testing::P1;
        use crate::testing::P2;

        let player = Entity::PLACEHOLDER;
        let happened = events::DrawFromEmptyLibrary {
            player: players::PlayerId(player),
        };
        let template = EventTemplate::WinGame {
            who: events::DrawFromEmptyLibrary::PLAYER.into(),
        };
        let (mut game, _) = MockGame::new([P1, P2]);
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(
                    template.instantiate(&happened, &answers),
                    rules::AnyEvent::WinGame(events::WinGame {
                        player: players::PlayerId(player)
                    })
                );
                assert_eq!(
                    EventTemplate::LoseGame {
                        who: events::DrawFromEmptyLibrary::PLAYER.into(),
                    }
                    .instantiate(&happened, &answers),
                    rules::AnyEvent::LoseGame(events::LoseGame {
                        player: players::PlayerId(player)
                    })
                );
            })
            .unwrap();
    }
}
