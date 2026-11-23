use crate::cards;
use crate::testing;

pub fn mana_ability(card: testing::TestCard, produces: cards::ManaType) -> testing::TestAbility {
    testing::TestAbility::Mana {
        source: *card,
        produces,
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::World;

    use crate::cards;
    use crate::testing;

    use super::mana_ability;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let entity = world.spawn_empty().id();
        let card = testing::TestCard(entity);

        assert_eq!(
            mana_ability(card, cards::ManaType::from(cards::Color::Blue)),
            testing::TestAbility::Mana {
                source: entity,
                produces: cards::ManaType::from(cards::Color::Blue),
            }
        );
    }
}
