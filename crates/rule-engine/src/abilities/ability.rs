use crate::abilities;
use crate::cards;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ability {
    Keyword(cards::Keyword),
    Triggered(abilities::TriggeredAbility),
}

impl From<cards::Keyword> for Ability {
    fn from(keyword: cards::Keyword) -> Self {
        Self::Keyword(keyword)
    }
}

impl From<abilities::TriggeredAbility> for Ability {
    fn from(ability: abilities::TriggeredAbility) -> Self {
        Self::Triggered(ability)
    }
}
