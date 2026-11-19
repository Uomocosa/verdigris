use crate::SPELL;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::testing;

pub const CANCEL: testing::Card = testing::Card {
    name: "Cancel",
    definition: || {
        cards::instant("Cancel", cost![1, U, U])
            .spell(effects::counter().to(effects::target(1, SPELL)))
    },
};

#[cfg(test)]
mod tests {
    use crate::SPELL;
    use crate::cards;
    use crate::cost;
    use crate::effects;

    use super::CANCEL;

    #[test]
    fn test_usage() {
        let counter = (CANCEL.definition)();
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
