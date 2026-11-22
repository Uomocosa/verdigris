use crate::abilities;
use crate::keywords;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ability {
    Keyword(keywords::StaticKeyword),
    Triggered(abilities::TriggeredAbility),
    Static(abilities::Continuous),
}

impl From<keywords::StaticKeyword> for Ability {
    fn from(keyword: keywords::StaticKeyword) -> Self {
        Self::Keyword(keyword)
    }
}

impl From<abilities::TriggeredAbility> for Ability {
    fn from(ability: abilities::TriggeredAbility) -> Self {
        Self::Triggered(ability)
    }
}

impl From<abilities::Continuous> for Ability {
    fn from(ability: abilities::Continuous) -> Self {
        Self::Static(ability)
    }
}
