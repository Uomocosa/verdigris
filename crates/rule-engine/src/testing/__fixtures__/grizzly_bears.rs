use crate::cards;
use crate::cost;
use crate::pt;

#[must_use]
pub fn grizzly_bears() -> cards::CardDefinition {
    cards::creature("Grizzly Bears", cost![1, G], pt!(2, 2))
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::pt;

    use super::grizzly_bears;

    #[test]
    fn test_usage() {
        let bears = grizzly_bears();
        assert_eq!(*bears.name, "Grizzly Bears");
        assert_eq!(bears.types, vec![cards::CardType::Creature]);
        assert_eq!(bears.power_toughness, Some(pt!(2, 2)));
    }
}
