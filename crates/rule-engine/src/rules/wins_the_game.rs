use crate::rules;
use crate::rules::traits::legality_rule::LegalityRule;

pub struct WinsTheGame;

#[rustfmt::skip]
impl LegalityRule for WinsTheGame {
    type Ctx = rules::PlayerCtx;
    const ID: rules::RuleId = rules::RuleId("104.2");
    fn clauses() -> Vec<rules::Clause> { Vec::new() }
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::rules::traits::legality_rule::LegalityRule;

    use super::WinsTheGame;

    #[test]
    fn test_usage() {
        assert_eq!(WinsTheGame::ID, rules::RuleId("104.2"));
        assert_eq!(WinsTheGame::clauses(), Vec::new());
    }
}
