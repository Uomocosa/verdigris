use bevy::prelude::EntityCommands;

use crate::abilities;
use crate::cards;

pub fn write_triggered_markers(
    characteristics: &cards::Characteristics,
    entity: &mut EntityCommands,
    current: cards::CurrentMarkersItem<'_, '_, '_>,
) {
    let mut desired_enters = None;
    let mut desired_at_step = None;
    if characteristics.on_battlefield {
        for ability in &characteristics.triggered_abilities {
            match ability.event {
                abilities::TriggerEvent::Enters(_) => {
                    if desired_enters.is_none() {
                        desired_enters = Some(cards::TriggeredEnters(*ability));
                    }
                }
                abilities::TriggerEvent::BeginningOfStep(_, _)
                | abilities::TriggerEvent::BeginningOfNextCleanup => {
                    if desired_at_step.is_none() {
                        desired_at_step = Some(cards::TriggeredAtStep(*ability));
                    }
                }
                abilities::TriggerEvent::Dies | abilities::TriggerEvent::Attacks => {}
            }
        }
    }
    if current.enters != desired_enters.as_ref() {
        if let Some(enters) = desired_enters {
            entity.insert(enters);
        } else {
            entity.remove::<cards::TriggeredEnters>();
        }
    }
    if current.at_step != desired_at_step.as_ref() {
        if let Some(at_step) = desired_at_step {
            entity.insert(at_step);
        } else {
            entity.remove::<cards::TriggeredAtStep>();
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{App, Commands, Query, Update};

    use crate::cards;
    use crate::testing;

    use super::write_triggered_markers;

    #[test]
    fn test_usage() {
        let mut app = App::new();
        let warden = (testing::SOUL_WARDEN.definition)();
        let characteristics = cards::Characteristics::from_printed(&warden, true).with_effects(&[]);
        let entity = app.world_mut().spawn(warden).id();
        app.add_systems(
            Update,
            move |mut commands: Commands, markers: Query<cards::CurrentMarkers<'_>>| {
                let current = markers.get(entity).expect("test spawns the card");
                write_triggered_markers(&characteristics, &mut commands.entity(entity), current);
            },
        );
        app.update();

        let world = app.world();
        assert!(world.get::<cards::TriggeredEnters>(entity).is_some());
        assert!(world.get::<cards::TriggeredAtStep>(entity).is_none());
    }
}
