use crate::cards;
use crate::cost;
use crate::pt;

#[must_use]
pub fn darksteel_myr() -> cards::CardDefinition {
    cards::artifact_creature("Darksteel Myr", cost![3], pt!(0, 1)).keyword(cards::indestructible())
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::cost;
    use crate::pt;

    use super::darksteel_myr;

    #[test]
    fn test_usage() {
        let myr = darksteel_myr();
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
