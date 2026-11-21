use crate::cards;

pub fn abilities<const N: usize>(
    mut definition: cards::CardDefinition,
    list: [cards::Ability; N],
) -> cards::CardDefinition {
    for ability in list {
        match ability {
            cards::Ability::Keyword(keyword) => definition.keywords.push(keyword),
            cards::Ability::Triggered(ability) => definition.triggered_abilities.push(ability),
        }
    }
    definition
}

#[cfg(test)]
mod tests {
    use crate::ANOTHER_CREATURE;
    use crate::YOU;
    use crate::cards;
    use crate::effects;
    use crate::testing;

    use super::abilities;

    #[test]
    fn test_usage() {
        let triggered = cards::whenever(
            ANOTHER_CREATURE.enters(),
            effects::gain_life(1).to_player(YOU),
        );
        let definition = (testing::GRIZZLY_BEARS.definition)();
        let definition = abilities(
            definition,
            [
                cards::Ability::from(cards::vigilance()),
                cards::Ability::from(triggered),
            ],
        );
        assert_eq!(definition.keywords, vec![cards::Keyword::Vigilance]);
        assert_eq!(definition.triggered_abilities, [triggered]);
    }
}
