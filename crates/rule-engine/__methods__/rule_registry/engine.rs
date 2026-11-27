use std::collections::HashSet;

use crate::LOSES_LIFE;
use crate::events;
use crate::rules;
use crate::rules::traits::event::Event;
use crate::rules::traits::hook_event::HookEvent;
use crate::rules::traits::value_rule::ValueRule;
#[cfg(any(test, feature = "testing"))]
use crate::testing;
use crate::{ASSIGNS_AGAIN, FIRST_COMBAT_DAMAGE_STEP};

pub fn engine() -> rules::RuleRegistry {
    let mut events = HashSet::<rules::EventId>::new();
    let mut restrictions = HashSet::<rules::RestrictionId>::new();
    let mut duplicates = Vec::<rules::EventId>::new();
    let mut ids = vec![
        events::AddPhase::ID,
        events::AssignCombatDamage::ID,
        events::BecomeTarget::ID,
        events::Cast::ID,
        events::Counter::ID,
        events::CreateTokens::ID,
        events::DealDamage::ID,
        events::DeclareAttacker::ID,
        events::DeclareAttackers::ID,
        events::DeclareBlocker::ID,
        events::Destroy::ID,
        events::Discard::ID,
        events::DiscardCard::ID,
        events::Draw::ID,
        events::DrawFromEmptyLibrary::ID,
        events::EnterTheBattlefield::ID,
        events::Exile::ID,
        events::GainLife::ID,
        events::JoinCombatDamageStep::ID,
        events::LoseGame::ID,
        events::LoseLife::ID,
        events::LoseUnspentMana::ID,
        events::Mill::ID,
        events::Play::ID,
        events::PutIntoGraveyard::ID,
        events::Tap::ID,
        events::TapAttacker::ID,
        events::Untap::ID,
        events::WinGame::ID,
        events::OpeningHandAction::ID,
        events::BeginTheGameOnTheBattlefield::ID,
    ];
    let mut lists = vec![
        events::AddPhase::restrictions(),
        events::AssignCombatDamage::restrictions(),
        events::BecomeTarget::restrictions(),
        events::Cast::restrictions(),
        events::Counter::restrictions(),
        events::CreateTokens::restrictions(),
        events::DealDamage::restrictions(),
        events::DeclareAttacker::restrictions(),
        events::DeclareAttackers::restrictions(),
        events::DeclareBlocker::restrictions(),
        events::Destroy::restrictions(),
        events::Discard::restrictions(),
        events::DiscardCard::restrictions(),
        events::Draw::restrictions(),
        events::DrawFromEmptyLibrary::restrictions(),
        events::EnterTheBattlefield::restrictions(),
        events::Exile::restrictions(),
        events::GainLife::restrictions(),
        events::JoinCombatDamageStep::restrictions(),
        events::LoseGame::restrictions(),
        events::LoseLife::restrictions(),
        events::LoseUnspentMana::restrictions(),
        events::Mill::restrictions(),
        events::Play::restrictions(),
        events::PutIntoGraveyard::restrictions(),
        events::Tap::restrictions(),
        events::TapAttacker::restrictions(),
        events::Untap::restrictions(),
        events::WinGame::restrictions(),
        events::OpeningHandAction::restrictions(),
        events::BeginTheGameOnTheBattlefield::restrictions(),
    ];
    #[cfg(any(test, feature = "testing"))]
    {
        ids.push(testing::TestRule::ID);
        lists.push(testing::TestRule::restrictions());
        ids.push(testing::AbsoluteRule::ID);
        lists.push(testing::AbsoluteRule::restrictions());
        ids.push(testing::RestrictedRule::ID);
        lists.push(testing::RestrictedRule::restrictions());
        ids.push(testing::TestStep::ID);
        lists.push(testing::TestStep::restrictions());
    }
    for (id, list) in ids.into_iter().zip(lists) {
        duplicates.extend((!events.insert(id)).then_some(id));
        restrictions.extend(list.iter().map(|restriction| restriction.id));
    }
    restrictions.extend(rules::RuleRegistry::keyword_restriction_ids());
    let results = HashSet::from([LOSES_LIFE]);
    let mut definitions = HashSet::from([rules::LETHAL_DAMAGE]);
    definitions.extend([FIRST_COMBAT_DAMAGE_STEP, ASSIGNS_AGAIN]);
    let values = HashSet::from([rules::LandPlays::ID, rules::MaximumHandSize::ID]);
    rules::RuleRegistry {
        events,
        results,
        restrictions,
        definitions,
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
