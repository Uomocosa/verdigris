use crate::rules;

pub const fn definition(
    definition_id: rules::DefinitionId,
    text: &'static str,
    condition: rules::Condition,
) -> rules::Definition {
    rules::Definition {
        id: definition_id,
        text,
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
            !rules::has::<cards::Tapped>(),
        );
        assert_eq!(built.id, rules::DefinitionId("glossary/lethal-damage"));
        assert_eq!(
            built.text,
            "An amount of damage greater than or equal to a creature's toughness."
        );
        assert_eq!(built.condition, !rules::has::<cards::Tapped>());
    }
}
