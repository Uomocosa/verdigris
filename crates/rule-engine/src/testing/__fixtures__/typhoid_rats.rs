use crate::cards;
use crate::cost;
use crate::pt;

#[must_use]
pub fn typhoid_rats() -> cards::CardDefinition {
    cards::creature("Typhoid Rats", cost![B], pt!(1, 1)).keyword(cards::deathtouch())
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::cost;
    use crate::pt;

    use super::typhoid_rats;

    #[test]
    fn test_usage() {
        let rats = typhoid_rats();
        assert_eq!(*rats.name, "Typhoid Rats");
        assert_eq!(rats.types, vec![cards::CardType::Creature]);
        assert_eq!(rats.cost, cost![B]);
        assert_eq!(rats.power_toughness, Some(pt!(1, 1)));
        assert_eq!(rats.keywords, vec![cards::Keyword::Deathtouch]);
    }
}
