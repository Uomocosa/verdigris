use crate::cards;
use crate::cost;
use crate::pt;
use crate::testing;

pub const DARKSTEEL_MYR: testing::Card = testing::Card {
    name: testing::TestCardName("Darksteel Myr"),
    definition: || {
        cards::artifact_creature("Darksteel Myr", cost![3], pt!(0, 1))
            .keyword(cards::indestructible())
    },
};

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::cost;
    use crate::pt;

    use super::DARKSTEEL_MYR;

    #[test]
    fn test_usage() {
        let myr = (DARKSTEEL_MYR.definition)();
        assert_eq!(*myr.name, "Darksteel Myr");
        assert_eq!(
            myr.types,
            vec![cards::CardType::Artifact, cards::CardType::Creature]
        );
        assert_eq!(myr.cost, cost![3]);
        assert_eq!(myr.power_toughness, Some(pt!(0, 1)));
        assert_eq!(myr.keywords, vec![cards::Keyword::Indestructible]);
    }
}
