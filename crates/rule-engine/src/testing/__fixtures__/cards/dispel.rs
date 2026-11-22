use crate::INSTANT_SPELL;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::testing;

pub const DISPEL: testing::Card = testing::Card {
    name: testing::TestCardName("Dispel"),
    definition: || {
        cards::instant("Dispel", cost![U])
            .spell(effects::counter().to(effects::target(1, INSTANT_SPELL)))
    },
};

#[cfg(test)]
mod tests {
    use crate::INSTANT_SPELL;
    use crate::cards;
    use crate::cost;
    use crate::effects;

    use super::DISPEL;

    #[test]
    fn test_usage() {
        let counter = (DISPEL.definition)();
        assert_eq!(*counter.name, "Dispel");
        assert_eq!(counter.type_line.types, vec![cards::CardType::Instant]);
        assert_eq!(counter.cost, cost![U]);
        assert_eq!(counter.colors(), vec![cards::Color::Blue]);
        assert_eq!(
            counter.spell_ability().copied(),
            Some(effects::counter().to(effects::target(1, INSTANT_SPELL)))
        );
    }
}
