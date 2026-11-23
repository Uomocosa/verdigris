use crate::rules;
use crate::rules::traits::value_rule::ValueRule;

pub struct LandPlays;

#[rustfmt::skip]
impl ValueRule for LandPlays {
    const ID: rules::RuleId = rules::RuleId("305.2");
    fn default() -> rules::Limit { rules::Limit::At(1) }
}
