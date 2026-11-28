use atomic_delegate_macros::atomic_delegates;
use derive_more::From;

use crate::abilities;
use crate::effects;
use crate::keywords;

#[derive(Debug, Clone, PartialEq, Eq, From)]
pub enum Ability {
    Spell(effects::Effect),
    Triggered(abilities::TriggeredAbility),
    Static(abilities::StaticAbility),
    Activated(abilities::ActivatedAbility),
}

impl From<keywords::Keyword> for Ability {
    fn from(keyword: keywords::Keyword) -> Self {
        Self::Static(abilities::StaticAbility::Keyword(keyword))
    }
}

impl From<abilities::Continuous> for Ability {
    fn from(ability: abilities::Continuous) -> Self {
        Self::Static(abilities::StaticAbility::Continuous(Box::new(ability)))
    }
}

impl From<abilities::Replacement> for Ability {
    fn from(replacement: abilities::Replacement) -> Self {
        Self::Static(abilities::StaticAbility::Replacement(replacement))
    }
}

#[atomic_delegates]
impl Ability {
    #[must_use]
    pub fn with_card(self, card: &'static str) -> Self {}
}

#[cfg(test)]
mod tests {
    use crate::PLAYER;
    use crate::abilities;
    use crate::effects;
    use crate::events;
    use crate::rules;
    use crate::statics;

    use super::Ability;

    #[test]
    fn test_usage() {
        let stamped =
            Ability::from(statics::all(PLAYER).cant::<events::Draw>(events::Draw::PLAYER))
                .with_card("test_card");
        let abilities::Ability::Static(abilities::StaticAbility::Continuous(continuous)) = stamped
        else {
            panic!("with_card keeps a continuous ability");
        };
        let effects::Modification::Rule(rules::RuleChange {
            change: rules::Change::Restrict { restriction, .. },
            ..
        }) = &continuous.modification
        else {
            panic!("a cant ability restricts an event");
        };
        assert_eq!(restriction.id.card, "test_card");
    }
}
