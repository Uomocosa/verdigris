use crate::PLAYER;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::testing;

pub const CARESS_OF_PHYREXIA: testing::Card = testing::Card {
    name: testing::TestCardName("Caress of Phyrexia"),
    definition: || {
        cards::sorcery("Caress of Phyrexia", cost![3, B, B]).spell(
            effects::draw(3)
                .and(effects::lose_life(3))
                .and(effects::poison(3))
                .to(effects::target(1, PLAYER)),
        )
    },
};

#[cfg(test)]
mod tests {
    use crate::PLAYER;
    use crate::cards;
    use crate::cost;
    use crate::effects;

    use super::CARESS_OF_PHYREXIA;

    #[test]
    fn test_usage() {
        let spell = (CARESS_OF_PHYREXIA.definition)();
        assert_eq!(*spell.name, "Caress of Phyrexia");
        assert_eq!(spell.types, vec![cards::CardType::Sorcery]);
        assert_eq!(spell.cost, cost![3, B, B]);
        assert_eq!(spell.colors(), vec![cards::Color::Black]);
        assert_eq!(
            spell.effect,
            Some(
                effects::draw(3)
                    .and(effects::lose_life(3))
                    .and(effects::poison(3))
                    .to(effects::target(1, PLAYER))
            )
        );
    }
}
