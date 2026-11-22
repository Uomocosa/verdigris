use crate::abilities;
use crate::keywords;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ability {
    Keyword(keywords::Keyword),
    Triggered(abilities::TriggeredAbility),
    Static(abilities::Static),
}

impl From<keywords::Keyword> for Ability {
    fn from(keyword: keywords::Keyword) -> Self {
        Self::Keyword(keyword)
    }
}

impl From<abilities::TriggeredAbility> for Ability {
    fn from(ability: abilities::TriggeredAbility) -> Self {
        Self::Triggered(ability)
    }
}

impl From<abilities::Static> for Ability {
    fn from(ability: abilities::Static) -> Self {
        Self::Static(ability)
    }
}
