use crate::rules;

pub const fn definition(
    definition_id: rules::DefinitionId,
    condition: rules::Condition,
) -> rules::Definition {
    rules::Definition {
        id: definition_id,
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
        let built = definition(id, !rules::has::<cards::Tapped>());
        assert_eq!(built.id, rules::DefinitionId("glossary/lethal-damage"));
        assert_eq!(built.condition, !rules::has::<cards::Tapped>());
    }
}
