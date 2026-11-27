use std::collections::HashSet;

use atomic_delegate_macros::atomic_delegates;

use crate::rules;

pub struct RuleRegistry {
    pub(crate) events: HashSet<rules::EventId>,
    pub(crate) results: HashSet<rules::ResultId>,
    pub(crate) restrictions: HashSet<rules::RestrictionId>,
    pub(crate) definitions: HashSet<rules::DefinitionId>,
    pub(crate) values: HashSet<rules::RuleId>,
    pub duplicates: Vec<rules::EventId>,
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
            rules::Target::Result(id) => self.results.contains(id),
            rules::Target::Restriction(key) => self.restrictions.contains(key),
            rules::Target::Definition(id) => self.definitions.contains(id),
            rules::Target::Value(id) => self.values.contains(id),
        }
    }
}

#[atomic_delegates]
impl RuleRegistry {
    #[must_use]
    pub(crate) fn keyword_restriction_ids() -> HashSet<rules::RestrictionId> {}
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::abilities;
    use crate::effects;
    use crate::events;
    use crate::rules;
    use crate::testing;

    #[test]
    fn test_usage() {
        let registry = rules::RuleRegistry::engine();
        assert!(registry.resolves(&rules::Target::event::<events::Draw>()));
        assert!(registry.resolves(&rules::Target::Restriction(
            events::declare_attackers::SUMMONING_SICK
        )));
        assert!(registry.resolves(&rules::Target::value::<rules::MaximumHandSize>()));
        assert_eq!(registry.duplicates, Vec::<rules::EventId>::new());
        assert!(!registry.resolves(&rules::Target::event::<events::PutCounters>()));
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
                            modifications.extend(keyword.abilities());
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

    #[test]
    fn no_fixture_card_has_two_optional_hooks_on_one_event() {
        for card in testing::ALL_CARDS {
            let definition = (card.definition)();
            let mut may_hooks_by_target = HashMap::new();
            for ability in &definition.text {
                let mut modifications = Vec::new();
                if let abilities::Ability::Static(static_ability) = ability {
                    match static_ability {
                        abilities::StaticAbility::Continuous(continuous) => {
                            modifications.push(continuous.modification.clone());
                        }
                        abilities::StaticAbility::Keyword(keyword) => {
                            modifications.extend(keyword.abilities());
                        }
                        abilities::StaticAbility::Replacement(replacement) => {
                            if let abilities::Replacement::Rule(change) = replacement {
                                modifications.push(effects::Modification::Rule(change.clone()));
                            }
                        }
                    }
                }
                for modification in modifications {
                    if let effects::Modification::Rule(rules::RuleChange { target, change }) =
                        modification
                        && matches!(
                            change,
                            rules::Change::Hook {
                                then: rules::Then::May(_),
                                ..
                            }
                        )
                    {
                        *may_hooks_by_target.entry(target).or_insert(0) += 1;
                    }
                }
            }
            for (target, count) in &may_hooks_by_target {
                assert!(
                    *count <= 1,
                    "card {} has {} optional hooks on {:?}",
                    card.name,
                    count,
                    target
                );
            }
        }
    }
}
