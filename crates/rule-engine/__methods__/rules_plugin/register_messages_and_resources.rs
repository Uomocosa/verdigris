use bevy::prelude::App;

use crate::Attempt;
use crate::May;
use crate::Resolve;
use crate::StateBasedActionsPerformed;
use crate::actions;
use crate::cards;
use crate::effects;
use crate::events;
use crate::rules;
use crate::stack;
use crate::turns;

pub fn register_messages_and_resources(app: &mut App) {
    app.add_message::<actions::PlayerAction>()
        .add_message::<actions::Rejected>()
        .add_message::<actions::Ask>()
        .add_message::<actions::Answered>()
        .add_message::<stack::Resolution>()
        .add_message::<Attempt<events::LoseGame>>()
        .add_message::<Attempt<events::WinGame>>()
        .add_message::<Attempt<events::DrawFromEmptyLibrary>>()
        .add_message::<Attempt<events::Draw>>()
        .add_message::<Attempt<events::LoseLife>>()
        .add_message::<Attempt<events::Discard>>()
        .add_message::<Attempt<events::GainLife>>()
        .add_message::<Attempt<events::Mill>>()
        .add_message::<Attempt<events::CreateTokens>>()
        .add_message::<Attempt<events::Destroy>>()
        .add_message::<Attempt<events::Counter>>()
        .add_message::<Attempt<events::DealDamage>>()
        .add_message::<Attempt<events::EnterBattlefield>>()
        .add_message::<Attempt<events::BeginTheGameOnTheBattlefield>>()
        .add_message::<Attempt<events::OpeningHandAction>>()
        .add_message::<Attempt<events::Untap>>()
        .add_message::<Attempt<events::TapAttacker>>()
        .add_message::<Attempt<events::PutIntoGraveyard>>()
        .add_message::<Resolve<events::LoseGame>>()
        .add_message::<Resolve<events::WinGame>>()
        .add_message::<Resolve<events::DrawFromEmptyLibrary>>()
        .add_message::<Resolve<events::Draw>>()
        .add_message::<Resolve<events::LoseLife>>()
        .add_message::<Resolve<events::Discard>>()
        .add_message::<Resolve<events::GainLife>>()
        .add_message::<Resolve<events::Mill>>()
        .add_message::<Resolve<events::CreateTokens>>()
        .add_message::<Resolve<events::Destroy>>()
        .add_message::<Resolve<events::Counter>>()
        .add_message::<Resolve<events::DealDamage>>()
        .add_message::<Resolve<events::EnterBattlefield>>()
        .add_message::<Resolve<events::BeginTheGameOnTheBattlefield>>()
        .add_message::<Resolve<events::OpeningHandAction>>()
        .add_message::<Resolve<events::Untap>>()
        .add_message::<Resolve<events::TapAttacker>>()
        .add_message::<Resolve<events::PutIntoGraveyard>>()
        .add_message::<May<events::Draw>>()
        .add_message::<May<events::Discard>>()
        .add_message::<May<events::Mill>>()
        .add_message::<May<events::CreateTokens>>()
        .add_message::<May<events::Counter>>()
        .add_message::<May<events::DealDamage>>()
        .add_message::<May<events::LoseLife>>()
        .add_message::<May<events::GainLife>>()
        .add_message::<May<events::EnterBattlefield>>()
        .add_message::<May<events::TapAttacker>>()
        .add_message::<May<events::Destroy>>()
        .add_message::<May<events::PutIntoGraveyard>>()
        .add_message::<May<events::DrawFromEmptyLibrary>>()
        .add_message::<May<events::LoseGame>>()
        .add_message::<May<events::WinGame>>()
        .add_message::<May<events::Untap>>()
        .add_message::<May<events::OpeningHandAction>>()
        .add_message::<May<events::BeginTheGameOnTheBattlefield>>()
        .add_message::<effects::PowerToughnessModified>()
        .add_message::<effects::KeywordGranted>()
        .add_message::<cards::Entered>()
        .add_message::<turns::StepStarted>()
        .add_message::<turns::AllPlayersPassed>()
        .add_message::<turns::MulligansComplete>()
        .add_message::<turns::OpeningHandActionsComplete>();
    app.init_resource::<actions::PendingChoice>();
    app.init_resource::<actions::AskQueue>();
    app.insert_resource(actions::PendingMayAsks::<events::OpeningHandAction>(
        Vec::new(),
    ));
    app.init_resource::<turns::CombatDamageSteps>();
    app.init_resource::<turns::CombatDamageParticipants>();
    app.init_resource::<StateBasedActionsPerformed>();
    app.init_resource::<effects::Timestamp>();
    app.init_resource::<stack::TriggerQueue>();
    let mut registry = rules::PredicateRegistry::default();
    {
        let world = app.world_mut();
        registry.register_predicate_component::<cards::Tapped>(world);
        registry.register_predicate_component::<cards::Attacking>(world);
        registry.register_predicate_component::<cards::Creature>(world);
        registry.register_predicate_component::<cards::Flying>(world);
        registry.register_predicate_component::<cards::Reach>(world);
        registry.register_predicate_component::<cards::Vigilance>(world);
        registry.register_predicate_component::<cards::Defender>(world);
        registry.register_predicate_component::<cards::SummoningSick>(world);
        registry.register_predicate_component::<cards::OnBattlefield>(world);
        registry.register_entity_component::<cards::Owner>(world);
        registry.register_entity_component::<cards::Blocking>(world);
        registry.register_entity_component::<cards::Enchanting>(world);
    }
    app.world_mut().spawn((registry,));
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
