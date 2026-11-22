use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::pt;
use crate::testing;

pub const DARKSTEEL_MYR: testing::Card = testing::Card {
    name: testing::TestCardName("Darksteel Myr"),
    definition: || {
        card! {
            cards::artifact_creature("Darksteel Myr", cost![3], pt!(0, 1));
            keywords::indestructible(),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::cost;
    use crate::keywords;
    use crate::pt;

    use super::DARKSTEEL_MYR;

    #[test]
    fn test_usage() {
        let myr = (DARKSTEEL_MYR.definition)();
        assert_eq!(*myr.name, "Darksteel Myr");
        assert_eq!(
            myr.type_line.types,
            vec![cards::CardType::Artifact, cards::CardType::Creature]
        );
        assert_eq!(myr.cost, cost![3]);
        assert_eq!(myr.power_toughness, Some(pt!(0, 1)));
        assert_eq!(
            myr.keywords().collect::<Vec<_>>(),
            vec![keywords::StaticKeyword::Indestructible]
        );
    }
}
