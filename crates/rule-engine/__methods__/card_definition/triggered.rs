use crate::abilities;
use crate::cards;

pub fn triggered(
    mut definition: cards::CardDefinition,
    ability: abilities::TriggeredAbility,
) -> cards::CardDefinition {
    definition.text.push(abilities::Ability::Triggered(ability));
    definition
}

#[cfg(test)]
mod tests {
    use crate::ANOTHER_CREATURE;
    use crate::YOU;
    use crate::abilities;
    use crate::effects;
    use crate::testing;

    use super::triggered;

    #[test]
    fn test_usage() {
        let definition = (testing::GRIZZLY_BEARS.definition)();
        let ability = abilities::TriggeredAbility {
            event: ANOTHER_CREATURE.enters(),
            intervening_if: None,
            effect: effects::gain_life(1).to_player(YOU),
        };
        assert_eq!(
            triggered(definition, ability.clone())
                .triggered_abilities()
                .cloned()
                .collect::<Vec<_>>(),
            [ability]
        );
    }
}
