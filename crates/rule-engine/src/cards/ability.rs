use crate::cards;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ability {
    Keyword(cards::Keyword),
    Triggered(cards::TriggeredAbility),
}

impl From<cards::Keyword> for Ability {
    fn from(keyword: cards::Keyword) -> Self {
        Self::Keyword(keyword)
    }
}

impl From<cards::TriggeredAbility> for Ability {
    fn from(ability: cards::TriggeredAbility) -> Self {
        Self::Triggered(ability)
    }
}
