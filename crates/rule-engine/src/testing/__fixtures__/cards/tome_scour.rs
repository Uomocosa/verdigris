use crate::PLAYER;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::testing;

pub const TOME_SCOUR: testing::Card = testing::Card {
    name: testing::TestCardName("Tome Scour"),
    definition: || {
        cards::sorcery("Tome Scour", cost![U])
            .spell(effects::mills(5).to(effects::target(1, PLAYER)))
    },
};

#[cfg(test)]
mod tests {
    use crate::PLAYER;
    use crate::cards;
    use crate::cost;
    use crate::effects;

    use super::TOME_SCOUR;

    #[test]
    fn test_usage() {
        let spell = (TOME_SCOUR.definition)();
        assert_eq!(*spell.name, "Tome Scour");
        assert_eq!(spell.type_line.types, vec![cards::CardType::Sorcery]);
        assert_eq!(spell.cost, cost![U]);
        assert_eq!(spell.colors(), vec![cards::Color::Blue]);
        assert_eq!(
            spell.effect,
            Some(effects::mills(5).to(effects::target(1, PLAYER)))
        );
    }
}
