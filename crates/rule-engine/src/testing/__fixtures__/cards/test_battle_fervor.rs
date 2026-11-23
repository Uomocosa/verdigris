use crate::CREATURE;
use crate::YOU;
use crate::card;
use crate::cards;
use crate::cost;
use crate::statics;
use crate::testing;

pub const TEST_BATTLE_FERVOR: testing::Card = testing::Card {
    name: testing::TestCardName("Test Battle Fervor"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Battle Fervor", cost![1, R]);
            statics::all(CREATURE.controlled_by(YOU)).attack_as_though_it_had_haste(),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::YOU;
    use crate::abilities;
    use crate::cards;
    use crate::cost;
    use crate::effects;
    use crate::rules;

    use super::TEST_BATTLE_FERVOR;

    #[test]
    fn test_usage() {
        let fervor = (TEST_BATTLE_FERVOR.definition)();
        assert_eq!(*fervor.name, "Test Battle Fervor");
        assert_eq!(fervor.type_line.types, vec![cards::CardType::Enchantment]);
        assert_eq!(fervor.cost, cost![1, R]);
        assert_eq!(fervor.colors(), vec![cards::Color::Red]);
        assert_eq!(
            fervor.continuous_abilities().cloned().collect::<Vec<_>>(),
            [abilities::Continuous {
                filter: CREATURE.controlled_by(YOU),
                modification: effects::Modification::AsThough(
                    rules::declare_attacker::SUMMONING_SICK.key
                ),
            }]
        );
    }
}
