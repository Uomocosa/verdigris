use crate::CREATURE;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::keywords;
use crate::testing;

pub const BLADEBRAND: testing::Card = testing::Card {
    name: testing::TestCardName("Bladebrand"),
    definition: || {
        cards::instant("Bladebrand", cost![1, B]).spell(
            effects::gains(keywords::deathtouch())
                .to(effects::target(1, CREATURE))
                .and(effects::draw(1)),
        )
    },
};

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::cards;
    use crate::cost;
    use crate::effects;
    use crate::keywords;

    use super::BLADEBRAND;

    #[test]
    fn test_usage() {
        let bladebrand = (BLADEBRAND.definition)();
        assert_eq!(*bladebrand.name, "Bladebrand");
        assert_eq!(bladebrand.type_line.types, vec![cards::CardType::Instant]);
        assert_eq!(bladebrand.cost, cost![1, B]);
        assert_eq!(bladebrand.colors(), vec![cards::Color::Black]);
        assert_eq!(
            bladebrand.spell_ability().copied(),
            Some(
                effects::gains(keywords::deathtouch())
                    .to(effects::target(1, CREATURE))
                    .and(effects::draw(1))
            )
        );
    }
}
