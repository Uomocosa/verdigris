use bevy::prelude::App;

use crate::Attempt;
use crate::May;
use crate::Resolve;
use crate::StateBasedActionsPerformed;
use crate::actions;
use crate::cards;
use crate::effects;
use crate::rules;
use crate::stack;
use crate::turns;

pub fn register_messages_and_resources(app: &mut App) {
    app.add_message::<actions::PlayerAction>()
        .add_message::<actions::Rejected>()
        .add_message::<actions::Ask>()
        .add_message::<actions::Answered>()
        .add_message::<stack::Resolution>()
        .add_message::<Attempt<rules::LoseGame>>()
        .add_message::<Attempt<rules::WinGame>>()
        .add_message::<Attempt<rules::DrawFromEmptyLibrary>>()
        .add_message::<Attempt<rules::Draw>>()
        .add_message::<Attempt<rules::LoseLife>>()
        .add_message::<Attempt<rules::Discard>>()
        .add_message::<Attempt<rules::GainLife>>()
        .add_message::<Attempt<rules::Mill>>()
        .add_message::<Attempt<rules::CreateTokens>>()
        .add_message::<Attempt<rules::Destroy>>()
        .add_message::<Attempt<rules::Counter>>()
        .add_message::<Attempt<rules::DealDamage>>()
        .add_message::<Attempt<rules::EnterBattlefield>>()
        .add_message::<Attempt<rules::BeginTheGameOnTheBattlefield>>()
        .add_message::<Attempt<rules::OpeningHandAction>>()
        .add_message::<Attempt<rules::Untap>>()
        .add_message::<Attempt<rules::TapAttacker>>()
        .add_message::<Attempt<rules::PutIntoGraveyard>>()
        .add_message::<Resolve<rules::LoseGame>>()
        .add_message::<Resolve<rules::WinGame>>()
        .add_message::<Resolve<rules::DrawFromEmptyLibrary>>()
        .add_message::<Resolve<rules::Draw>>()
        .add_message::<Resolve<rules::LoseLife>>()
        .add_message::<Resolve<rules::Discard>>()
        .add_message::<Resolve<rules::GainLife>>()
        .add_message::<Resolve<rules::Mill>>()
        .add_message::<Resolve<rules::CreateTokens>>()
        .add_message::<Resolve<rules::Destroy>>()
        .add_message::<Resolve<rules::Counter>>()
        .add_message::<Resolve<rules::DealDamage>>()
        .add_message::<Resolve<rules::EnterBattlefield>>()
        .add_message::<Resolve<rules::BeginTheGameOnTheBattlefield>>()
        .add_message::<Resolve<rules::OpeningHandAction>>()
        .add_message::<Resolve<rules::Untap>>()
        .add_message::<Resolve<rules::TapAttacker>>()
        .add_message::<Resolve<rules::PutIntoGraveyard>>()
        .add_message::<May<rules::Draw>>()
        .add_message::<May<rules::Discard>>()
        .add_message::<May<rules::Mill>>()
        .add_message::<May<rules::CreateTokens>>()
        .add_message::<May<rules::Counter>>()
        .add_message::<May<rules::DealDamage>>()
        .add_message::<May<rules::LoseLife>>()
        .add_message::<May<rules::GainLife>>()
        .add_message::<May<rules::EnterBattlefield>>()
        .add_message::<May<rules::TapAttacker>>()
        .add_message::<May<rules::Destroy>>()
        .add_message::<May<rules::PutIntoGraveyard>>()
        .add_message::<May<rules::DrawFromEmptyLibrary>>()
        .add_message::<May<rules::LoseGame>>()
        .add_message::<May<rules::WinGame>>()
        .add_message::<May<rules::Untap>>()
        .add_message::<May<rules::OpeningHandAction>>()
        .add_message::<May<rules::BeginTheGameOnTheBattlefield>>()
        .add_message::<effects::PowerToughnessModified>()
        .add_message::<effects::KeywordGranted>()
        .add_message::<cards::Entered>()
        .add_message::<turns::StepStarted>()
        .add_message::<turns::AllPlayersPassed>()
        .add_message::<turns::MulligansComplete>()
        .add_message::<turns::OpeningHandActionsComplete>();
    app.init_resource::<actions::PendingChoice>();
    app.init_resource::<actions::AskQueue>();
    app.insert_resource(actions::PendingMayAsks::<rules::OpeningHandAction>(
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
