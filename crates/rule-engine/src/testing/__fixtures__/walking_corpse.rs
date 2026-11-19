use crate::cards;
use crate::cost;
use crate::pt;

#[must_use]
pub fn walking_corpse() -> cards::CardDefinition {
    cards::creature("Walking Corpse", cost![1, B], pt!(2, 2))
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::cost;
    use crate::pt;

    use super::walking_corpse;

    #[test]
    fn test_usage() {
        let corpse = walking_corpse();
        assert_eq!(*corpse.name, "Walking Corpse");
        assert_eq!(corpse.types, vec![cards::CardType::Creature]);
        assert_eq!(corpse.power_toughness, Some(pt!(2, 2)));
        assert_eq!(corpse.cost, cost![1, B]);
        assert_eq!(corpse.colors(), vec![cards::Color::Black]);
    }
}
