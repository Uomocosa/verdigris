use atomic_delegate_macros::atomic_delegates;

use crate::rules;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause {
    pub key: rules::ClauseKey,
    pub text: &'static str,
    pub role: rules::Role,
    pub condition: rules::Predicate,
    pub absolute: bool,
}

#[atomic_delegates]
impl Clause {
    #[must_use]
    pub fn absolute(self) -> Self {}
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::rules;

    use super::Clause;

    #[test]
    fn test_usage() {
        let clause = rules::ClauseId::<rules::DeclareBlocker>::new("509.1a/untapped").clause(
            "The chosen creatures must be untapped",
            rules::Role::Subject,
            !rules::has::<cards::Tapped>(),
        );
        assert_eq!(clause.key, rules::ClauseKey("509.1a/untapped"));
        assert!(!clause.absolute);
        let absolute = Clause {
            absolute: true,
            ..clause
        };
        assert!(absolute.absolute);
    }
}
