use crate::rules;
use crate::rules::traits::value_rule::ValueRule;

pub struct LethalDamage;

#[rustfmt::skip]
impl ValueRule for LethalDamage {
    const ID: rules::RuleId = rules::RuleId("120.6");
    fn default() -> rules::Limit { rules::Limit::NoMaximum }
}
