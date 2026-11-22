use crate::cards;

pub fn from_printed(
    definition: &cards::CardDefinition,
    on_battlefield: bool,
) -> cards::Characteristics {
    cards::Characteristics {
        on_battlefield,
        power_toughness: definition.power_toughness,
        keywords: definition.keywords().collect(),
        triggered_abilities: definition.triggered_abilities().copied().collect(),
    }
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::cost;
    use crate::pt;

    use super::from_printed;

    #[test]
    fn test_usage() {
        let bears = cards::creature("Grizzly Bears", cost![1, G], pt!(2, 2));
        let characteristics = from_printed(&bears, true);
        assert!(characteristics.on_battlefield);
        assert_eq!(characteristics.power_toughness, Some(pt!(2, 2)));
    }
}
