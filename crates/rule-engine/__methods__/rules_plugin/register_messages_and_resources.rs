use bevy::prelude::App;

use crate::StateBasedActionsPerformed;
use crate::actions;
use crate::effects;
use crate::events;
use crate::rules::traits::event::Event;
use crate::rules::traits::hook_event::HookEvent;
use crate::stack;
use crate::turns;

pub fn register_messages_and_resources(app: &mut App) {
    app.add_message::<actions::PlayerAction>()
        .add_message::<actions::Rejected>()
        .add_message::<actions::Ask>()
        .add_message::<actions::Answered>()
        .add_message::<stack::Resolution>();
    events::AddPhase::register(app);
    events::LoseGame::register(app);
    events::LoseUnspentMana::register(app);
    events::WinGame::register(app);
    events::DrawFromEmptyLibrary::register(app);
    events::Draw::register(app);
    events::LoseLife::register(app);
    events::Discard::register(app);
    app.add_message::<actions::DiscardRequest>();
    events::GainLife::register(app);
    events::PutCounters::register(app);
    events::Mill::register(app);
    events::Create::register(app);
    events::Destroy::register(app);
    events::Counter::register(app);
    events::DealDamage::register(app);
    events::EnterTheBattlefield::register(app);
    events::Exile::register(app);
    events::BeginTheGameOnTheBattlefield::register(app);
    events::OpeningHandAction::register(app);
    events::Untap::register(app);
    events::DeclareAttackers::register(app);
    events::DeclareBlocker::register(app);
    events::Tap::register(app);
    events::PutIntoGraveyard::register(app);
    app.add_message::<effects::PowerToughnessModified>()
        .add_message::<effects::KeywordGranted>()
        .add_message::<turns::StepStarted>()
        .add_message::<turns::NextStep>()
        .add_message::<turns::AllPlayersPassed>()
        .add_message::<turns::MulligansComplete>()
        .add_message::<turns::TurnStarted>();
    app.init_resource::<actions::PendingChoice>();
    app.init_resource::<actions::LegalActions>();
    app.init_resource::<actions::AskQueue>();
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
