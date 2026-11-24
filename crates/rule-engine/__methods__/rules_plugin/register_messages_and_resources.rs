use bevy::prelude::App;

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
        .add_message::<rules::Would<rules::LoseGame>>()
        .add_message::<rules::Would<rules::WinGame>>()
        .add_message::<rules::Would<rules::DrawFromEmptyLibrary>>()
        .add_message::<rules::Would<rules::Draw>>()
        .add_message::<rules::Would<rules::LoseLife>>()
        .add_message::<rules::Would<rules::Discard>>()
        .add_message::<rules::Would<rules::GainLife>>()
        .add_message::<rules::Would<rules::Mill>>()
        .add_message::<rules::Would<rules::CreateTokens>>()
        .add_message::<rules::Would<rules::Destroy>>()
        .add_message::<rules::Would<rules::Counter>>()
        .add_message::<rules::Would<rules::DealDamage>>()
        .add_message::<rules::Would<rules::EnterBattlefield>>()
        .add_message::<rules::Would<rules::Untap>>()
        .add_message::<rules::Would<rules::TapAttacker>>()
        .add_message::<rules::Would<rules::PutIntoGraveyard>>()
        .add_message::<rules::Proceeds<rules::LoseGame>>()
        .add_message::<rules::Proceeds<rules::WinGame>>()
        .add_message::<rules::Proceeds<rules::DrawFromEmptyLibrary>>()
        .add_message::<rules::Proceeds<rules::Draw>>()
        .add_message::<rules::Proceeds<rules::LoseLife>>()
        .add_message::<rules::Proceeds<rules::Discard>>()
        .add_message::<rules::Proceeds<rules::GainLife>>()
        .add_message::<rules::Proceeds<rules::Mill>>()
        .add_message::<rules::Proceeds<rules::CreateTokens>>()
        .add_message::<rules::Proceeds<rules::Destroy>>()
        .add_message::<rules::Proceeds<rules::Counter>>()
        .add_message::<rules::Proceeds<rules::DealDamage>>()
        .add_message::<rules::Proceeds<rules::EnterBattlefield>>()
        .add_message::<rules::Proceeds<rules::Untap>>()
        .add_message::<rules::Proceeds<rules::TapAttacker>>()
        .add_message::<rules::Proceeds<rules::PutIntoGraveyard>>()
        .add_message::<rules::Ask<rules::Draw>>()
        .add_message::<rules::Ask<rules::Discard>>()
        .add_message::<rules::Ask<rules::Mill>>()
        .add_message::<rules::Ask<rules::CreateTokens>>()
        .add_message::<rules::Ask<rules::Counter>>()
        .add_message::<rules::Ask<rules::DealDamage>>()
        .add_message::<rules::Ask<rules::LoseLife>>()
        .add_message::<rules::Ask<rules::GainLife>>()
        .add_message::<rules::Ask<rules::EnterBattlefield>>()
        .add_message::<rules::Ask<rules::TapAttacker>>()
        .add_message::<rules::Ask<rules::Destroy>>()
        .add_message::<rules::Ask<rules::PutIntoGraveyard>>()
        .add_message::<rules::Ask<rules::DrawFromEmptyLibrary>>()
        .add_message::<rules::Ask<rules::LoseGame>>()
        .add_message::<rules::Ask<rules::WinGame>>()
        .add_message::<rules::Ask<rules::Untap>>()
        .add_message::<effects::PowerToughnessModified>()
        .add_message::<effects::KeywordGranted>()
        .add_message::<cards::Entered>()
        .add_message::<turns::StepStarted>()
        .add_message::<turns::AllPlayersPassed>();
    app.init_resource::<actions::PendingChoice>();
    app.init_resource::<actions::AskQueue>();
    app.init_resource::<actions::OpeningHandWindow>();
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
