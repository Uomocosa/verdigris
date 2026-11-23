use crate::rules;

pub fn absolute(clause: rules::Clause) -> rules::Clause {
    rules::Clause {
        absolute: true,
        ..clause
    }
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::rules;

    use super::absolute;

    #[test]
    fn test_usage() {
        let clause = rules::ClauseId::<rules::DeclareBlocker>::new("509.1a/untapped").clause(
            "The chosen creatures must be untapped",
            rules::Role::Subject,
            !rules::has::<cards::Tapped>(),
        );
        assert!(!clause.absolute);
        let made_absolute = absolute(clause);
        assert!(made_absolute.absolute);
        assert_eq!(made_absolute.key, rules::ClauseKey("509.1a/untapped"));
    }
}
