use crate::CREATURE;
use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::statics;
use crate::testing;

pub const HOLY_STRENGTH: testing::Card = testing::Card {
    name: testing::TestCardName("Holy Strength"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Holy Strength", cost![W])
                .subtype(cards::EnchantmentType::Aura.into());
            keywords::enchant(CREATURE),
            statics::enchanted().get_pt(1, 2),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::abilities;
    use crate::cards;
    use crate::cost;
    use crate::effects;
    use crate::keywords;

    use super::HOLY_STRENGTH;

    #[test]
    fn test_usage() {
        let strength = (HOLY_STRENGTH.definition)();
        assert_eq!(*strength.name, "Holy Strength");
        assert_eq!(strength.type_line.types, vec![cards::CardType::Enchantment]);
        assert_eq!(
            strength.type_line.subtypes,
            vec![cards::Subtype::Enchantment(cards::EnchantmentType::Aura)]
        );
        assert_eq!(strength.cost, cost![W]);
        assert_eq!(strength.colors(), vec![cards::Color::White]);
        assert_eq!(strength.enchant(), Some(CREATURE));
        assert_eq!(
            strength.keywords().collect::<Vec<_>>(),
            vec![keywords::StaticKeyword::Enchant(CREATURE)]
        );
        assert_eq!(
            strength.continuous_abilities().copied().collect::<Vec<_>>(),
            [abilities::Continuous {
                filter: effects::Filter {
                    object: Some(effects::ObjectRelation::EnchantedObject),
                    ..CREATURE
                },
                change: effects::Change::Characteristic(effects::Layered::ModifyPT {
                    power: 1,
                    toughness: 2,
                }),
            }]
        );
    }
}
