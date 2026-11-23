use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;

use crate::abilities;
use crate::effects;
use crate::keywords;
use crate::rules;
use crate::rules::traits::rule_question::RuleQuestion;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct All(pub effects::Filter);

#[atomic_delegates]
impl All {
    #[must_use]
    pub fn gain(self, keyword: keywords::StaticKeyword) -> abilities::Continuous {}

    #[must_use]
    pub fn get_pt(self, power: i32, toughness: i32) -> abilities::Continuous {}

    #[must_use]
    pub fn lose(self, keyword: keywords::StaticKeyword) -> abilities::Continuous {}

    #[must_use]
    pub fn attack_as_though_it_had_haste(self) -> abilities::Continuous {}

    #[must_use]
    pub fn must_attack(self) -> abilities::Continuous {}

    #[must_use]
    pub fn must_block(self) -> abilities::Continuous {}

    #[must_use]
    pub fn attack_without_tapping(self) -> abilities::Continuous {}

    #[must_use]
    pub fn tap_when_attacking(self) -> abilities::Continuous {}

    #[must_use]
    pub fn rule<Q: RuleQuestion>(self, answer: Q::Answer) -> abilities::Continuous {}

    #[must_use]
    pub fn waive<Ctx>(self, clause: rules::ClauseId<Ctx>) -> abilities::Continuous {}

    #[must_use]
    pub fn restrict<R: rules::traits::legality_rule::LegalityRule>(
        self,
        requirement: rules::Predicate,
    ) -> abilities::Continuous {
    }

    #[must_use]
    pub fn demand<R: rules::traits::legality_rule::LegalityRule>(self) -> abilities::Continuous {}

    #[must_use]
    pub fn skip(self, step: rules::StepId) -> abilities::Continuous {}
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
