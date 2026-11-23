use crate::rules;
use crate::rules::traits::legality_rule::LegalityRule;

pub struct LosesTheGame;

#[rustfmt::skip]
impl LegalityRule for LosesTheGame {
    type Ctx = rules::PlayerCtx;
    const ID: rules::RuleId = rules::RuleId("104.3");
    fn clauses() -> Vec<rules::Clause> { Vec::new() }
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::rules::traits::legality_rule::LegalityRule;

    use super::LosesTheGame;

    #[test]
    fn test_usage() {
        assert_eq!(LosesTheGame::ID, rules::RuleId("104.3"));
        assert_eq!(LosesTheGame::clauses(), Vec::new());
    }
}
