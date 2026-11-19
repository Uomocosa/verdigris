use crate::CREATURE_SPELL;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::testing;

pub const ESSENCE_SCATTER: testing::Card = testing::Card {
    name: "Essence Scatter",
    definition: || {
        cards::instant("Essence Scatter", cost![1, U])
            .spell(effects::counter().to(effects::target(1, CREATURE_SPELL)))
    },
};

#[cfg(test)]
mod tests {
    use crate::CREATURE_SPELL;
    use crate::cards;
    use crate::cost;
    use crate::effects;

    use super::ESSENCE_SCATTER;

    #[test]
    fn test_usage() {
        let counter = (ESSENCE_SCATTER.definition)();
        assert_eq!(*counter.name, "Essence Scatter");
        assert_eq!(counter.types, vec![cards::CardType::Instant]);
        assert_eq!(counter.cost, cost![1, U]);
        assert_eq!(counter.colors(), vec![cards::Color::Blue]);
        assert_eq!(
            counter.effect,
            Some(effects::counter().to(effects::target(1, CREATURE_SPELL)))
        );
    }
}
