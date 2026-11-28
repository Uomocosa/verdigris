use crate::rules;
use crate::rules::traits::value_rule::ValueRule;

pub struct BlocksPerCreature;

#[rustfmt::skip]
impl ValueRule for BlocksPerCreature {
    const ID: rules::RuleId = rules::RuleId("509.1a/blocks-per-creature");
    fn default() -> rules::Limit { rules::Limit::At(1) }
}
