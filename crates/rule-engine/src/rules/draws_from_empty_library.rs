use crate::rules;
use crate::rules::traits::legality_rule::LegalityRule;

pub struct DrawsFromEmptyLibrary;

#[rustfmt::skip]
impl LegalityRule for DrawsFromEmptyLibrary {
    type Ctx = rules::PlayerCtx;
    const ID: rules::RuleId = rules::RuleId("121.4");
    fn clauses() -> Vec<rules::Clause> { Vec::new() }
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::rules::traits::legality_rule::LegalityRule;

    use super::DrawsFromEmptyLibrary;

    #[test]
    fn test_usage() {
        assert_eq!(DrawsFromEmptyLibrary::ID, rules::RuleId("121.4"));
        assert_eq!(DrawsFromEmptyLibrary::clauses(), Vec::new());
    }
}
