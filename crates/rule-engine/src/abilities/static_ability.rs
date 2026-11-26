use crate::abilities;
use crate::keywords;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StaticAbility {
    Keyword(keywords::Keyword),
    Continuous(abilities::Continuous),
    Replacement(abilities::Replacement),
}

impl From<keywords::Keyword> for StaticAbility {
    fn from(keyword: keywords::Keyword) -> Self {
        Self::Keyword(keyword)
    }
}

impl From<abilities::Continuous> for StaticAbility {
    fn from(ability: abilities::Continuous) -> Self {
        Self::Continuous(ability)
    }
}

impl From<abilities::Replacement> for StaticAbility {
    fn from(replacement: abilities::Replacement) -> Self {
        Self::Replacement(replacement)
    }
}
