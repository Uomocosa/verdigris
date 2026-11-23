use crate::ENCHANTMENT;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::testing;

pub const DEMYSTIFY: testing::Card = testing::Card {
    name: testing::TestCardName("Demystify"),
    definition: || {
        cards::instant("Demystify", cost![W])
            .spell(effects::destroy().to(effects::target(1, ENCHANTMENT)))
    },
};

#[cfg(test)]
mod tests {
    use crate::ENCHANTMENT;
    use crate::cards;
    use crate::cost;
    use crate::effects;

    use super::DEMYSTIFY;

    #[test]
    fn test_usage() {
        let demystify = (DEMYSTIFY.definition)();
        assert_eq!(*demystify.name, "Demystify");
        assert_eq!(demystify.type_line.types, vec![cards::CardType::Instant]);
        assert_eq!(demystify.cost, cost![W]);
        assert_eq!(demystify.colors(), vec![cards::Color::White]);
        assert_eq!(
            demystify.spell_ability().copied(),
            Some(effects::destroy().to(effects::target(1, ENCHANTMENT)))
        );
    }
}
