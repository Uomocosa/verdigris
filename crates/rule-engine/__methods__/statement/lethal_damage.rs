use crate::rules;

pub fn lethal_damage() -> rules::Definition {
    rules::LETHAL_DAMAGE.definition(
        "An amount of damage greater than or equal to a creature's toughness.",
        rules::Condition::EventFieldAtLeastRemainingToughness(rules::Field::Amount),
    )
}

#[cfg(test)]
mod tests {
    use crate::rules;

    use super::lethal_damage;

    #[test]
    fn test_usage() {
        let definition = lethal_damage();
        assert_eq!(definition, rules::Definition::lethal_damage());
        assert_eq!(definition.id, rules::LETHAL_DAMAGE);
        assert_eq!(
            definition.text,
            "An amount of damage greater than or equal to a creature's toughness."
        );
        assert_eq!(
            definition.condition,
            rules::Condition::EventFieldAtLeastRemainingToughness(rules::Field::Amount)
        );
    }
}
