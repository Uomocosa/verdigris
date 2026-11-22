use bevy::prelude::App;

use crate::StateBasedActionsPerformed;
use crate::actions;
use crate::cards;
use crate::damage;
use crate::effects;
use crate::players;
use crate::stack;
use crate::turns;

pub fn register_messages_and_resources(app: &mut App) {
    app.add_message::<actions::PlayerAction>()
        .add_message::<actions::Rejected>()
        .add_message::<stack::Resolution>()
        .add_message::<stack::Countered>()
        .add_message::<damage::DamageDealt>()
        .add_message::<damage::Destruction>()
        .add_message::<players::DrawRequested>()
        .add_message::<players::LifeLost>()
        .add_message::<players::LifeGained>()
        .add_message::<players::DiscardRequested>()
        .add_message::<effects::MillRequested>()
        .add_message::<effects::PowerToughnessModified>()
        .add_message::<effects::KeywordGranted>()
        .add_message::<cards::Entered>()
        .add_message::<turns::StepStarted>()
        .add_message::<turns::AllPlayersPassed>();
    app.init_resource::<actions::PendingChoice>();
    app.init_resource::<turns::CombatDamageSteps>();
    app.init_resource::<turns::CombatDamageParticipants>();
    app.init_resource::<StateBasedActionsPerformed>();
    app.init_resource::<effects::Timestamp>();
    app.init_resource::<stack::TriggerQueue>();
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{App, Messages};

    use crate::actions;
    use crate::turns;

    use super::register_messages_and_resources;

    #[test]
    fn test_usage() {
        let mut app = App::new();
        register_messages_and_resources(&mut app);
        assert!(app.world().get_resource::<Messages<actions::PlayerAction>>().is_some());
        assert!(app.world().get_resource::<Messages<turns::StepStarted>>().is_some());
        assert!(app.world().get_resource::<actions::PendingChoice>().is_some());
    }
}
