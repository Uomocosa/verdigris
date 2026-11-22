use crate::NONCREATURE_SPELL;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::testing;

pub const NEGATE: testing::Card = testing::Card {
    name: testing::TestCardName("Negate"),
    definition: || {
        cards::instant("Negate", cost![1, U])
            .spell(effects::counter().to(effects::target(1, NONCREATURE_SPELL)))
    },
};

#[cfg(test)]
mod tests {
    use crate::NONCREATURE_SPELL;
    use crate::cards;
    use crate::cost;
    use crate::effects;

    use super::NEGATE;

    #[test]
    fn test_usage() {
        let counter = (NEGATE.definition)();
        assert_eq!(*counter.name, "Negate");
        assert_eq!(counter.type_line.types, vec![cards::CardType::Instant]);
        assert_eq!(counter.cost, cost![1, U]);
        assert_eq!(counter.colors(), vec![cards::Color::Blue]);
        assert_eq!(
            counter.effect,
            Some(effects::counter().to(effects::target(1, NONCREATURE_SPELL)))
        );
    }
}
