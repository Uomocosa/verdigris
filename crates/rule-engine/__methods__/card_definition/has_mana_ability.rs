use crate::abilities;
use crate::cards;

pub fn has_mana_ability(definition: &cards::CardDefinition) -> bool {
    definition.type_line.land_types().next().is_some()
        || definition
            .activated_abilities()
            .any(|ability| ability.cost == abilities::ActivationCost::Tap)
}

#[cfg(test)]
mod tests {
    use crate::testing;

    use super::has_mana_ability;

    #[test]
    fn test_usage() {
        let forest = (testing::FOREST.definition)();
        assert!(has_mana_ability(&forest));
        let tower = (testing::RELIQUARY_TOWER.definition)();
        assert!(has_mana_ability(&tower));
        let bears = (testing::GRIZZLY_BEARS.definition)();
        assert!(!has_mana_ability(&bears));
    }
}
