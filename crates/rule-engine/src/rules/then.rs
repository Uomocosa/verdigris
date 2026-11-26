use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::{Commands, MessageWriter};
use derive_more::From;

use crate::May;
use crate::StateBasedActionsPerformed;
use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, PartialEq, Eq, Hash, From)]
pub enum Then {
    Events(Vec<rules::EventTemplate>),
    May(rules::EventTemplate),
}

#[atomic_delegates]
impl Then {
    pub fn announce<E: Event>(
        self,
        event: &E,
        commands: &mut Commands,
        asks: &mut MessageWriter<May<E>>,
        state_based_actions_performed: &mut StateBasedActionsPerformed,
    ) {
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::{Commands, MessageReader, MessageWriter, Res, ResMut};

    use crate::Attempt;
    use crate::May;
    use crate::StateBasedActionsPerformed;
    use crate::events;
    use crate::rules;
    use crate::testing::*;

    use super::Then;

    #[test]
    fn test_usage() {
        let (mut game, [p1, _]) = MockGame::new([P1, P2]);
        let player = p1.id;
        let happened = events::DrawFromEmptyLibrary { player };
        let then = Then::Events(vec![rules::EventTemplate::WinGame {
            who: rules::Role::Player,
        }]);
        game.app
            .world_mut()
            .run_system_once(
                move |mut commands: Commands,
                      mut asks: MessageWriter<May<events::DrawFromEmptyLibrary>>,
                      mut performed: ResMut<StateBasedActionsPerformed>| {
                    then.clone().announce(&happened, &mut commands, &mut asks, &mut performed);
                },
            )
            .unwrap();
        game.app
            .world_mut()
            .run_system_once(
                move |mut attempts: MessageReader<Attempt<events::WinGame>>,
                      performed: Res<StateBasedActionsPerformed>| {
                    assert!(attempts.read().any(|attempt| **attempt == events::WinGame { player }));
                    assert!(**performed);
                },
            )
            .unwrap();
    }
}
