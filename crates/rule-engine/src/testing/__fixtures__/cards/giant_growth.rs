use crate::CREATURE;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::testing;

pub const GIANT_GROWTH: testing::Card = testing::Card {
    name: testing::TestCardName("Giant Growth"),
    definition: || {
        cards::instant("Giant Growth", cost![G])
            .spell(effects::gets(3, 3).to(effects::target(1, CREATURE)))
    },
};

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::cards;
    use crate::cost;
    use crate::effects;

    use super::GIANT_GROWTH;

    #[test]
    fn test_usage() {
        let growth = (GIANT_GROWTH.definition)();
        assert_eq!(*growth.name, "Giant Growth");
        assert_eq!(growth.type_line.types, vec![cards::CardType::Instant]);
        assert_eq!(growth.cost, cost![G]);
        assert_eq!(growth.colors(), vec![cards::Color::Green]);
        assert_eq!(
            growth.effect,
            Some(effects::gets(3, 3).to(effects::target(1, CREATURE)))
        );
    }
}
