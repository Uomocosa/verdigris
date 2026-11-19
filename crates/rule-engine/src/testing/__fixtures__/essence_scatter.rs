use crate::CREATURE_SPELL;
use crate::cards;
use crate::cost;
use crate::effects;

#[must_use]
pub fn essence_scatter() -> cards::CardDefinition {
    cards::instant("Essence Scatter", cost![1, U])
        .spell(effects::counter().to(effects::target(1, CREATURE_SPELL)))
}

#[cfg(test)]
mod tests {
    use crate::CREATURE_SPELL;
    use crate::cards;
    use crate::cost;
    use crate::effects;

    use super::essence_scatter;

    #[test]
    fn test_usage() {
        let counter = essence_scatter();
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
