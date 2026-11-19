use crate::cards;
use crate::cost;
use crate::effects;

#[must_use]
pub fn divination() -> cards::CardDefinition {
    cards::sorcery("Divination", cost![2, U]).spell(effects::draw(2))
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::cost;
    use crate::effects;

    use super::divination;

    #[test]
    fn test_usage() {
        let spell = divination();
        assert_eq!(*spell.name, "Divination");
        assert_eq!(spell.types, vec![cards::CardType::Sorcery]);
        assert_eq!(spell.cost, cost![2, U]);
        assert_eq!(spell.colors(), vec![cards::Color::Blue]);
        assert_eq!(spell.effect, Some(effects::draw(2)));
    }
}
