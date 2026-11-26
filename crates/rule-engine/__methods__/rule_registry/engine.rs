use std::collections::HashSet;

use crate::events;
use crate::rules;
use crate::rules::traits::event::Event;
use crate::rules::traits::hook_event::HookEvent;
use crate::rules::traits::value_rule::ValueRule;
#[cfg(any(test, feature = "testing"))]
use crate::testing;

pub fn engine() -> rules::RuleRegistry {
    let mut events = HashSet::<rules::EventId>::new();
    let mut restrictions = HashSet::<rules::RestrictionId>::new();
    let mut duplicates = Vec::<rules::EventId>::new();
    let mut register = |id: rules::EventId, list: Vec<rules::Restriction>| {
        if !events.insert(id) {
            duplicates.push(id);
        }
        for restriction in list {
            restrictions.insert(restriction.id);
        }
    };
    register(events::AddPhase::ID, events::AddPhase::restrictions());
    register(
        events::AssignCombatDamage::ID,
        events::AssignCombatDamage::restrictions(),
    );
    register(
        events::BecomeTarget::ID,
        events::BecomeTarget::restrictions(),
    );
    register(events::CastSpell::ID, events::CastSpell::restrictions());
    register(events::Counter::ID, events::Counter::restrictions());
    register(
        events::CreateTokens::ID,
        events::CreateTokens::restrictions(),
    );
    register(events::DealDamage::ID, events::DealDamage::restrictions());
    register(
        events::DeclareAttacker::ID,
        events::DeclareAttacker::restrictions(),
    );
    register(
        events::DeclareBlocker::ID,
        events::DeclareBlocker::restrictions(),
    );
    register(events::Destroy::ID, events::Destroy::restrictions());
    register(events::Discard::ID, events::Discard::restrictions());
    register(events::DiscardCard::ID, events::DiscardCard::restrictions());
    register(events::Draw::ID, events::Draw::restrictions());
    register(
        events::DrawFromEmptyLibrary::ID,
        events::DrawFromEmptyLibrary::restrictions(),
    );
    register(
        events::EnterBattlefield::ID,
        events::EnterBattlefield::restrictions(),
    );
    register(events::Exile::ID, events::Exile::restrictions());
    register(events::GainLife::ID, events::GainLife::restrictions());
    register(events::LoseGame::ID, events::LoseGame::restrictions());
    register(events::LoseLife::ID, events::LoseLife::restrictions());
    register(
        events::LoseUnspentMana::ID,
        events::LoseUnspentMana::restrictions(),
    );
    register(events::Mill::ID, events::Mill::restrictions());
    register(events::PlayLand::ID, events::PlayLand::restrictions());
    register(
        events::PutIntoGraveyard::ID,
        events::PutIntoGraveyard::restrictions(),
    );
    register(events::Tap::ID, events::Tap::restrictions());
    register(events::TapAttacker::ID, events::TapAttacker::restrictions());
    register(events::Untap::ID, events::Untap::restrictions());
    register(events::WinGame::ID, events::WinGame::restrictions());
    register(
        events::OpeningHandAction::ID,
        events::OpeningHandAction::restrictions(),
    );
    register(
        events::BeginTheGameOnTheBattlefield::ID,
        events::BeginTheGameOnTheBattlefield::restrictions(),
    );
    #[cfg(any(test, feature = "testing"))]
    {
        register(testing::TestRule::ID, testing::TestRule::restrictions());
        register(
            testing::AbsoluteRule::ID,
            testing::AbsoluteRule::restrictions(),
        );
        register(
            testing::RestrictedRule::ID,
            testing::RestrictedRule::restrictions(),
        );
        register(testing::TestStep::ID, testing::TestStep::restrictions());
    }
    let values = HashSet::from([
        rules::LandPlays::ID,
        rules::LethalDamage::ID,
        rules::MaximumHandSize::ID,
    ]);
    rules::RuleRegistry {
        events,
        restrictions,
        values,
        duplicates,
    }
}

#[cfg(test)]
mod tests {
    use crate::events;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::value_rule::ValueRule;

    use super::engine;

    #[test]
    fn test_usage() {
        let registry = engine();
        assert!(registry.events.contains(&events::Draw::ID));
        assert!(registry.restrictions.contains(&events::declare_attacker::SUMMONING_SICK));
        assert!(registry.values.contains(&rules::LandPlays::ID));
        assert_eq!(events::Draw::ID, rules::EventId("121.1/draw"));
    }
}
