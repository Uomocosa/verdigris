use crate::PLAYER;
use crate::YOU;
use crate::abilities;
use crate::card;
use crate::cards;
use crate::cost;
use crate::statics;
use crate::testing;

pub const LEYLINE_OF_ANTICIPATION: testing::Card = testing::Card {
    name: testing::TestCardName("Leyline of Anticipation"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Leyline of Anticipation", cost![2, U, U]);
            abilities::begin_the_game_with_it_on_the_battlefield(),
            statics::all(PLAYER.controlled_by(YOU)).cast_spells_as_though_they_had_flash(),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::PLAYER;
    use crate::YOU;
    use crate::abilities;
    use crate::cards;
    use crate::cost;
    use crate::effects;
    use crate::rules;

    use super::LEYLINE_OF_ANTICIPATION;

    #[test]
    fn test_usage() {
        let leyline = (LEYLINE_OF_ANTICIPATION.definition)();
        assert_eq!(*leyline.name, "Leyline of Anticipation");
        assert_eq!(leyline.type_line.types, vec![cards::CardType::Enchantment]);
        assert_eq!(leyline.cost, cost![2, U, U]);
        assert_eq!(leyline.colors(), vec![cards::Color::Blue]);
        assert!(leyline.begins_the_game_on_the_battlefield());
        assert_eq!(leyline.text.len(), 2);
        assert_eq!(
            leyline.continuous_abilities().cloned().collect::<Vec<_>>(),
            [abilities::Continuous {
                filter: PLAYER.controlled_by(YOU),
                modification: effects::Modification::Rule(rules::RuleChange {
                    target: rules::Target::Clause(rules::cast_spell::TIMING.key),
                    change: rules::Change::AsThough,
                }),
            }]
        );
    }
}
