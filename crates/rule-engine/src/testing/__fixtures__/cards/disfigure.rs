use crate::CREATURE;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::testing;

pub const DISFIGURE: testing::Card = testing::Card {
    name: testing::TestCardName("Disfigure"),
    definition: || {
        cards::instant("Disfigure", cost![B])
            .spell(effects::gets(-2, -2).to(effects::target(1, CREATURE)))
    },
};

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::cards;
    use crate::cost;
    use crate::effects;

    use super::DISFIGURE;

    #[test]
    fn test_usage() {
        let disfigure = (DISFIGURE.definition)();
        assert_eq!(*disfigure.name, "Disfigure");
        assert_eq!(disfigure.types, vec![cards::CardType::Instant]);
        assert_eq!(disfigure.cost, cost![B]);
        assert_eq!(disfigure.colors(), vec![cards::Color::Black]);
        assert_eq!(
            disfigure.effect,
            Some(effects::gets(-2, -2).to(effects::target(1, CREATURE)))
        );
    }
}
