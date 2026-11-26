use crate::rules;

pub const fn definition(
    definition_id: rules::DefinitionId,
    text: &'static str,
    role: rules::Role,
    condition: rules::Condition,
) -> rules::Definition {
    rules::Definition {
        id: definition_id,
        text,
        role,
        condition,
        unwaivable: false,
    }
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::rules;

    use super::definition;

    #[test]
    fn test_usage() {
        let id = rules::DefinitionId("glossary/lethal-damage");
        let built = definition(
            id,
            "An amount of damage greater than or equal to a creature's toughness.",
            rules::Role::Other,
            !rules::has::<cards::Tapped>(),
        );
        assert_eq!(built.id, rules::DefinitionId("glossary/lethal-damage"));
        assert_eq!(
            built.text,
            "An amount of damage greater than or equal to a creature's toughness."
        );
        assert_eq!(built.role, rules::Role::Other);
        assert_eq!(built.condition, !rules::has::<cards::Tapped>());
    }
}
