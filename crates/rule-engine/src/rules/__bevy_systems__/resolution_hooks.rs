use bevy::prelude::{Commands, MessageReader, MessageWriter, ResMut};

use crate::May;
use crate::Resolve;
use crate::StateBasedActionsPerformed;
use crate::rules;
use crate::rules::traits::event::Event;

pub fn resolution_hooks<E: Event>(
    mut resolved: MessageReader<Resolve<E>>,
    mut commands: Commands,
    mut asks: MessageWriter<May<E>>,
    mut state_based_actions_performed: ResMut<StateBasedActionsPerformed>,
    answers: rules::Rules,
) {
    for event in resolved.read() {
        for then in answers.hooks(&**event) {
            then.announce(
                &**event,
                &answers,
                &mut commands,
                &mut asks,
                &mut state_based_actions_performed,
            );
        }
    }
}
