use crate::cards;
use crate::cost;
use crate::pt;
use crate::testing;

pub const ENDLESS_ONE: testing::Card = testing::Card {
    name: testing::TestCardName("Endless One"),
    definition: || cards::creature("Endless One", cost![X], pt!(0, 0)),
};

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::cost;
    use crate::pt;

    use super::ENDLESS_ONE;

    #[test]
    fn test_usage() {
        let one = (ENDLESS_ONE.definition)();
        assert_eq!(*one.name, "Endless One");
        assert_eq!(one.type_line.types, vec![cards::CardType::Creature]);
        assert_eq!(one.cost, cost![X]);
        assert_eq!(one.power_toughness, Some(pt!(0, 0)));
    }
}
