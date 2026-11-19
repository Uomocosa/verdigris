use crate::SPELL;
use crate::cards;
use crate::cost;
use crate::effects;

#[must_use]
pub fn cancel() -> cards::CardDefinition {
    cards::instant("Cancel", cost![1, U, U]).spell(effects::counter().to(effects::target(1, SPELL)))
}

#[cfg(test)]
mod tests {
    use crate::SPELL;
    use crate::cards;
    use crate::cost;
    use crate::effects;

    use super::cancel;

    #[test]
    fn test_usage() {
        let counter = cancel();
        assert_eq!(*counter.name, "Cancel");
        assert_eq!(counter.types, vec![cards::CardType::Instant]);
        assert_eq!(counter.cost, cost![1, U, U]);
        assert_eq!(counter.colors(), vec![cards::Color::Blue]);
        assert_eq!(
            counter.effect,
            Some(effects::counter().to(effects::target(1, SPELL)))
        );
    }
}
