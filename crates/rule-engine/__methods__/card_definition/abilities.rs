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
            abilities::Ability::Static(ability) => definition.static_abilities.push(ability),
        }
    }
    definition
}

#[cfg(test)]
mod tests {
    use crate::ANOTHER_CREATURE;
    use crate::CREATURE;
    use crate::YOU;
    use crate::abilities;
    use crate::effects;
    use crate::keywords;
    use crate::statics;
    use crate::testing;

    use super::abilities as apply_abilities;

    #[test]
    fn test_usage() {
        let triggered = abilities::whenever(
            ANOTHER_CREATURE.enters(),
            effects::gain_life(1).to_player(YOU),
        );
        let definition = (testing::GRIZZLY_BEARS.definition)();
        let static_ability = statics::all(CREATURE).gain(keywords::vigilance());
        let definition = apply_abilities(
            definition,
            [
                abilities::Ability::from(keywords::vigilance()),
                abilities::Ability::from(triggered),
                abilities::Ability::from(static_ability),
            ],
        );
        assert_eq!(definition.keywords, vec![keywords::Keyword::Vigilance]);
        assert_eq!(definition.triggered_abilities, [triggered]);
        assert_eq!(definition.static_abilities, [static_ability]);
    }
}
