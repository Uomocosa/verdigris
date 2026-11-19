use crate::INSTANT_SPELL;
use crate::cards;
use crate::cost;
use crate::effects;

#[must_use]
pub fn dispel() -> cards::CardDefinition {
    cards::instant("Dispel", cost![U])
        .spell(effects::counter().to(effects::target(1, INSTANT_SPELL)))
}

#[cfg(test)]
mod tests {
    use crate::INSTANT_SPELL;
    use crate::cards;
    use crate::cost;
    use crate::effects;

    use super::dispel;

    #[test]
    fn test_usage() {
        let counter = dispel();
        assert_eq!(*counter.name, "Dispel");
        assert_eq!(counter.types, vec![cards::CardType::Instant]);
        assert_eq!(counter.cost, cost![U]);
        assert_eq!(counter.colors(), vec![cards::Color::Blue]);
        assert_eq!(
            counter.effect,
            Some(effects::counter().to(effects::target(1, INSTANT_SPELL)))
        );
    }
}
