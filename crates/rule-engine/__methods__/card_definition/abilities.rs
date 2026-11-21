use crate::abilities;
use crate::cards;

pub fn abilities<const N: usize>(
    mut definition: cards::CardDefinition,
    list: [abilities::Ability; N],
) -> cards::CardDefinition {
    for ability in list {
        match ability {
            abilities::Ability::Keyword(keyword) => definition.keywords.push(keyword),
            abilities::Ability::Triggered(ability) => definition.triggered_abilities.push(ability),
        }
    }
    definition
}

#[cfg(test)]
mod tests {
    use crate::ANOTHER_CREATURE;
    use crate::YOU;
    use crate::abilities;
    use crate::cards;
    use crate::effects;
    use crate::testing;

    use super::abilities as apply_abilities;

    #[test]
    fn test_usage() {
        let triggered = abilities::whenever(
            ANOTHER_CREATURE.enters(),
            effects::gain_life(1).to_player(YOU),
        );
        let definition = (testing::GRIZZLY_BEARS.definition)();
        let definition = apply_abilities(
            definition,
            [
                abilities::Ability::from(cards::vigilance()),
                abilities::Ability::from(triggered),
            ],
        );
        assert_eq!(definition.keywords, vec![cards::Keyword::Vigilance]);
        assert_eq!(definition.triggered_abilities, [triggered]);
    }
}
