use crate::rules;
use crate::rules::traits::value_rule::ValueRule;

pub struct MaximumHandSize;

#[rustfmt::skip]
impl ValueRule for MaximumHandSize {
    const ID: rules::RuleId = rules::RuleId("402.2");
    fn default() -> rules::Limit { rules::Limit::At(7) }
}
