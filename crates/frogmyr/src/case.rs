use atomic_delegate_macros::atomic_delegates;

use crate::Facts;
use crate::Program;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Case {
    pub name: &'static str,
    pub program: Program,
    pub facts: Facts,
    pub expected: Facts,
}

#[atomic_delegates]
impl Case {
    #[must_use]
    pub fn cases() -> Vec<Case> {}
    #[must_use]
    pub fn check() -> Result<(), String> {}
    #[must_use]
    pub fn transitive_closure() -> Case {}
    #[must_use]
    pub fn entity_traversal() -> Case {}
    #[must_use]
    pub fn stratified_negation() -> Case {}
    #[must_use]
    pub fn comparison_literal() -> Case {}
    #[must_use]
    pub fn constants_in_head_and_body() -> Case {}
    #[must_use]
    pub fn two_independent_strata() -> Case {}
    #[must_use]
    pub fn negated_relation_empty() -> Case {}
    #[must_use]
    pub fn unstratifiable() -> Case {}
    #[must_use]
    pub fn unsafe_head() -> Case {}
}

#[cfg(test)]
mod tests {
    use super::Case;

    #[test]
    fn test_usage() {
        let all = Case::cases();
        assert_eq!(all.len(), 9);
        assert_eq!(all[0].name, "transitive_closure");
    }
}
