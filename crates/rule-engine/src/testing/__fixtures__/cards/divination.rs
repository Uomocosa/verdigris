use crate::cards;
use crate::cost;
use crate::effects;
use crate::testing;

pub const DIVINATION: testing::Card = testing::Card {
    name: testing::TestCardName("Divination"),
    definition: || cards::sorcery("Divination", cost![2, U]).spell(effects::draw(2)),
};

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::cost;
    use crate::effects;

    use super::DIVINATION;

    #[test]
    fn test_usage() {
        let spell = (DIVINATION.definition)();
        assert_eq!(*spell.name, "Divination");
        assert_eq!(spell.type_line.types, vec![cards::CardType::Sorcery]);
        assert_eq!(spell.cost, cost![2, U]);
        assert_eq!(spell.colors(), vec![cards::Color::Blue]);
        assert_eq!(spell.effect, Some(effects::draw(2)));
    }
}
