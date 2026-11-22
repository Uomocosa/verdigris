use crate::CREATURE;
use crate::card;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::testing;

pub const HOLY_STRENGTH: testing::Card = testing::Card {
    name: testing::TestCardName("Holy Strength"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Holy Strength", cost![W])
                .aura(effects::gets(1, 2).to(effects::target(1, CREATURE)));
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::cards;
    use crate::cost;
    use crate::effects;

    use super::HOLY_STRENGTH;

    #[test]
    fn test_usage() {
        let strength = (HOLY_STRENGTH.definition)();
        assert_eq!(*strength.name, "Holy Strength");
        assert_eq!(strength.types, vec![cards::CardType::Enchantment]);
        assert_eq!(strength.cost, cost![W]);
        assert_eq!(strength.colors(), vec![cards::Color::White]);
        assert_eq!(
            strength.aura,
            Some(effects::gets(1, 2).to(effects::target(1, CREATURE)))
        );
    }
}
