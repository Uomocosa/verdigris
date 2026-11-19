use crate::cards;

pub fn creature(
    name: impl Into<cards::CardName>,
    cost: cards::Cost,
    power_toughness: cards::PowerToughness,
) -> cards::CardDefinition {
    cards::CardDefinition {
        name: name.into(),
        cost,
        types: vec![cards::CardType::Creature],
        power_toughness: Some(power_toughness),
        effect: None,
    }
}

pub fn instant(name: impl Into<cards::CardName>, cost: cards::Cost) -> cards::CardDefinition {
    cards::CardDefinition {
        name: name.into(),
        cost,
        types: vec![cards::CardType::Instant],
        power_toughness: None,
        effect: None,
    }
}
