use bevy::prelude::{Commands, MessageWriter};

use crate::May;
use crate::StateBasedActionsPerformed;
use crate::rules;
use crate::rules::traits::event::Event;

pub fn announce<E: Event>(
    then: rules::Then,
    event: &E,
    commands: &mut Commands,
    asks: &mut MessageWriter<May<E>>,
    state_based_actions_performed: &mut StateBasedActionsPerformed,
) {
    match then {
        rules::Then::Events(then) => {
            for template in then {
                template.instantiate(event).announce(commands);
            }
            **state_based_actions_performed = true;
        }
        rules::Then::May(then) => {
            asks.write(May {
                event: event.clone(),
                then,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::{Commands, MessageReader, MessageWriter, ResMut};

    use crate::May;
    use crate::StateBasedActionsPerformed;
    use crate::events;
    use crate::rules;
    use crate::testing::*;

    use super::announce;

    #[test]
    fn test_usage() {
        let (mut game, [p1, _]) = MockGame::new([P1, P2]);
        let player = p1.id;
        let happened = events::DrawFromEmptyLibrary(player);
        game.app
            .world_mut()
            .run_system_once(
                move |mut commands: Commands,
                      mut asks: MessageWriter<May<events::DrawFromEmptyLibrary>>,
                      mut performed: ResMut<StateBasedActionsPerformed>| {
                    announce(
                        rules::may(rules::EventTemplate::Mill {
                            who: rules::Role::Player,
                            amount: rules::Amount::Fixed(1),
                        }),
                        &happened,
                        &mut commands,
                        &mut asks,
                        &mut performed,
                    );
                },
            )
            .unwrap();
        game.app
            .world_mut()
            .run_system_once(
                move |mut asks: MessageReader<May<events::DrawFromEmptyLibrary>>| {
                    assert!(asks.read().any(|ask| {
                        ask.event == events::DrawFromEmptyLibrary(player)
                            && ask.then
                                == rules::EventTemplate::Mill {
                                    who: rules::Role::Player,
                                    amount: rules::Amount::Fixed(1),
                                }
                    }));
                },
            )
            .unwrap();
    }
}
