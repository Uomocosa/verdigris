use crate::rules;

pub const fn clause<Ctx>(
    clause_id: rules::ClauseId<Ctx>,
    text: &'static str,
    role: rules::Role,
    condition: rules::Predicate,
) -> rules::Clause {
    rules::Clause {
        key: clause_id.key,
        text,
        role,
        condition,
        absolute: false,
    }
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::rules;

    use super::clause;

    #[test]
    fn test_usage() {
        let id = rules::ClauseId::<rules::DeclareBlocker>::new("509.1a/untapped");
        let built = clause(
            id,
            "The chosen creatures must be untapped",
            rules::Role::Subject,
            !rules::has::<cards::Tapped>(),
        );
        assert_eq!(built.key, rules::ClauseKey("509.1a/untapped"));
        assert_eq!(built.text, "The chosen creatures must be untapped");
        assert_eq!(built.role, rules::Role::Subject);
        assert_eq!(built.condition, !rules::has::<cards::Tapped>());
    }
}
