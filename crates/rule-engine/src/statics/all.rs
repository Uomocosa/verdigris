use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;

use crate::abilities;
use crate::effects;
use crate::keywords;
use crate::rules::traits::rule_question::RuleQuestion;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct All(pub effects::Filter);

#[atomic_delegates]
impl All {
    #[must_use]
    pub fn gain(self, keyword: keywords::Keyword) -> abilities::Static {}

    #[must_use]
    pub fn get_pt(self, power: i32, toughness: i32) -> abilities::Static {}

    #[must_use]
    pub fn lose(self, keyword: keywords::Keyword) -> abilities::Static {}

    #[must_use]
    pub fn attack_as_though_it_had_haste(self) -> abilities::Static {}

    #[must_use]
    pub fn must_attack(self) -> abilities::Static {}

    #[must_use]
    pub fn must_block(self) -> abilities::Static {}

    #[must_use]
    pub fn attack_without_tapping(self) -> abilities::Static {}

    #[must_use]
    pub fn tap_when_attacking(self) -> abilities::Static {}

    #[must_use]
    pub fn rule<Q: RuleQuestion>(self, answer: Q::Answer) -> abilities::Static {}
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::keywords;
    use crate::statics;

    #[test]
    fn test_usage() {
        let builder = statics::all(CREATURE);
        assert_eq!(*builder, CREATURE);
        let ability = builder.gain(keywords::vigilance());
        assert_eq!(ability.filter, CREATURE);
    }
}
