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
    let mut clauses = HashSet::<rules::ClauseKey>::new();
    let mut register = |id: rules::EventId, list: Vec<rules::Clause>| {
        events.insert(id);
        for clause in list {
            clauses.insert(clause.key);
        }
    };
    register(events::CastSpell::ID, events::CastSpell::clauses());
    register(events::Counter::ID, events::Counter::clauses());
    register(events::CreateTokens::ID, events::CreateTokens::clauses());
    register(events::DealDamage::ID, events::DealDamage::clauses());
    register(
        events::DeclareAttacker::ID,
        events::DeclareAttacker::clauses(),
    );
    register(
        events::DeclareBlocker::ID,
        events::DeclareBlocker::clauses(),
    );
    register(events::Destroy::ID, events::Destroy::clauses());
    register(events::Discard::ID, events::Discard::clauses());
    register(events::Draw::ID, events::Draw::clauses());
    register(
        events::DrawFromEmptyLibrary::ID,
        events::DrawFromEmptyLibrary::clauses(),
    );
    register(
        events::EnterBattlefield::ID,
        events::EnterBattlefield::clauses(),
    );
    register(events::GainLife::ID, events::GainLife::clauses());
    register(events::LoseGame::ID, events::LoseGame::clauses());
    register(events::LoseLife::ID, events::LoseLife::clauses());
    register(
        events::LoseUnspentMana::ID,
        events::LoseUnspentMana::clauses(),
    );
    register(events::Mill::ID, events::Mill::clauses());
    register(events::PlayLand::ID, events::PlayLand::clauses());
    register(
        events::PutIntoGraveyard::ID,
        events::PutIntoGraveyard::clauses(),
    );
    register(events::Tap::ID, events::Tap::clauses());
    register(events::TapAttacker::ID, events::TapAttacker::clauses());
    register(events::Untap::ID, events::Untap::clauses());
    register(events::WinGame::ID, events::WinGame::clauses());
    register(
        events::OpeningHandAction::ID,
        events::OpeningHandAction::clauses(),
    );
    register(
        events::BeginTheGameOnTheBattlefield::ID,
        events::BeginTheGameOnTheBattlefield::clauses(),
    );
    #[cfg(any(test, feature = "testing"))]
    {
        register(testing::TestRule::ID, testing::TestRule::clauses());
        register(testing::AbsoluteRule::ID, testing::AbsoluteRule::clauses());
        register(
            testing::RestrictedRule::ID,
            testing::RestrictedRule::clauses(),
        );
        register(testing::TestStep::ID, testing::TestStep::clauses());
    }
    let values = HashSet::from([rules::LandPlays::ID, rules::MaximumHandSize::ID]);
    rules::RuleRegistry {
        events,
        clauses,
        values,
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
        assert!(registry.clauses.contains(&events::declare_attacker::SUMMONING_SICK.key));
        assert!(registry.values.contains(&rules::LandPlays::ID));
        assert_eq!(events::Draw::ID, rules::EventId("121.1"));
    }
}
