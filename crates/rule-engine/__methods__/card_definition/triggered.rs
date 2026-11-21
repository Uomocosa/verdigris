use crate::cards;

pub fn triggered(
    mut definition: cards::CardDefinition,
    ability: cards::TriggeredAbility,
) -> cards::CardDefinition {
    definition.triggered_abilities.push(ability);
    definition
}

#[cfg(test)]
mod tests {
    use crate::ANOTHER_CREATURE;
    use crate::YOU;
    use crate::cards;
    use crate::effects;
    use crate::testing;

    use super::triggered;

    #[test]
    fn test_usage() {
        let definition = (testing::GRIZZLY_BEARS.definition)();
        let ability = cards::TriggeredAbility {
            event: ANOTHER_CREATURE.enters(),
            intervening_if: None,
            effect: effects::gain_life(1).to_player(YOU),
        };
        assert_eq!(
            triggered(definition, ability).triggered_abilities,
            [ability]
        );
    }
}
