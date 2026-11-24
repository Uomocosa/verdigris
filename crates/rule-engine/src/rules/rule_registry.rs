use std::collections::HashSet;

use atomic_delegate_macros::atomic_delegates;

use crate::rules;

pub struct RuleRegistry {
    pub(crate) events: HashSet<rules::EventId>,
    pub(crate) clauses: HashSet<rules::ClauseKey>,
    pub(crate) values: HashSet<rules::RuleId>,
}

#[atomic_delegates]
impl RuleRegistry {
    #[must_use]
    pub fn engine() -> Self {}
}

impl RuleRegistry {
    #[must_use]
    pub fn resolves(&self, target: &rules::Target) -> bool {
        match target {
            rules::Target::Event(id) => self.events.contains(id),
            rules::Target::Clause(key) => self.clauses.contains(key),
            rules::Target::Value(id) => self.values.contains(id),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::abilities;
    use crate::effects;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::value_rule::ValueRule;
    use crate::testing;

    #[test]
    fn test_usage() {
        let registry = rules::RuleRegistry::engine();
        assert!(registry.resolves(&rules::Target::Event(rules::Draw::ID)));
        assert!(registry.resolves(&rules::Target::Clause(
            rules::declare_attacker::SUMMONING_SICK.key
        )));
        assert!(registry.resolves(&rules::Target::Value(rules::MaximumHandSize::ID)));
        assert!(!registry.resolves(&rules::Target::Event(rules::EventId("999.9"))));
    }

    #[test]
    fn every_fixture_card_targets_something_real() {
        let registry = rules::RuleRegistry::engine();
        for card in testing::ALL_CARDS {
            let definition = (card.definition)();
            for ability in &definition.text {
                let mut modifications = Vec::new();
                if let abilities::Ability::Static(static_ability) = ability {
                    match static_ability {
                        abilities::StaticAbility::Continuous(continuous) => {
                            modifications.push(continuous.modification.clone());
                        }
                        abilities::StaticAbility::Keyword(keyword) => {
                            modifications.extend(keyword.keyword_modifications());
                        }
                        abilities::StaticAbility::Replacement(replacement) => {
                            if let abilities::Replacement::Rule(change) = replacement {
                                modifications.push(effects::Modification::Rule(change.clone()));
                            }
                        }
                    }
                }
                for modification in modifications {
                    if let effects::Modification::Rule(change) = modification {
                        assert!(
                            registry.resolves(&change.target),
                            "card {} names dangling target {:?}",
                            card.name,
                            change.target
                        );
                    }
                }
            }
        }
    }
}
