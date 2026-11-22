use crate::ANY_TARGET;
use crate::cards;
use crate::effects;
use crate::testing;

pub const GUT_SHOT: testing::Card = testing::Card {
    name: testing::TestCardName("Gut Shot"),
    definition: || {
        cards::instant(
            "Gut Shot",
            cards::Cost::default().with_phyrexian(cards::Color::Red),
        )
        .spell(effects::damage(1).to(effects::target(1, ANY_TARGET)))
    },
};

#[cfg(test)]
mod tests {
    use crate::ANY_TARGET;
    use crate::cards;
    use crate::effects;

    use super::GUT_SHOT;

    #[test]
    fn test_usage() {
        let spell = (GUT_SHOT.definition)();
        assert_eq!(*spell.name, "Gut Shot");
        assert_eq!(spell.types, vec![cards::CardType::Instant]);
        assert_eq!(spell.cost.phyrexian, Some(cards::Color::Red));
        assert_eq!(spell.cost.red, 1);
        assert_eq!(spell.colors(), vec![cards::Color::Red]);
        assert_eq!(
            spell.effect,
            Some(effects::damage(1).to(effects::target(1, ANY_TARGET)))
        );
    }
}
