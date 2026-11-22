use derive_more::From;

use crate::abilities;
use crate::effects;
use crate::keywords;

#[derive(Debug, Clone, Copy, PartialEq, Eq, From)]
pub enum Ability {
    Spell(effects::Effect),
    Triggered(abilities::TriggeredAbility),
    Static(abilities::StaticAbility),
}

impl From<keywords::StaticKeyword> for Ability {
    fn from(keyword: keywords::StaticKeyword) -> Self {
        Self::Static(abilities::StaticAbility::Keyword(keyword))
    }
}

impl From<abilities::Continuous> for Ability {
    fn from(ability: abilities::Continuous) -> Self {
        Self::Static(abilities::StaticAbility::Continuous(ability))
    }
}

impl From<abilities::Replacement> for Ability {
    fn from(replacement: abilities::Replacement) -> Self {
        Self::Static(abilities::StaticAbility::Replacement(replacement))
    }
}
