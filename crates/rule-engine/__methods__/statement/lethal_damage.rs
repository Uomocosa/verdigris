use crate::events;
use crate::rules;

pub fn lethal_damage() -> rules::Definition {
    rules::LETHAL_DAMAGE.definition(
        rules::Condition::ArgAtLeastRemainingToughness(events::AssignCombatDamage::AMOUNT)
            .of(events::AssignCombatDamage::TARGET),
    )
}

#[cfg(test)]
mod tests {
    use crate::events;
    use crate::rules;

    use super::lethal_damage;

    #[test]
    fn test_usage() {
        let definition = lethal_damage();
        assert_eq!(definition, rules::Definition::lethal_damage());
        assert_eq!(definition.id, rules::LETHAL_DAMAGE);
        assert_eq!(
            definition.condition,
            rules::Condition::ArgAtLeastRemainingToughness(events::AssignCombatDamage::AMOUNT)
                .of(events::AssignCombatDamage::TARGET)
        );
    }
}
