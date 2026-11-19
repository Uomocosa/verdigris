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
        keywords: Vec::new(),
    }
}

pub fn artifact_creature(
    name: impl Into<cards::CardName>,
    cost: cards::Cost,
    power_toughness: cards::PowerToughness,
) -> cards::CardDefinition {
    cards::CardDefinition {
        name: name.into(),
        cost,
        types: vec![cards::CardType::Artifact, cards::CardType::Creature],
        power_toughness: Some(power_toughness),
        effect: None,
        keywords: Vec::new(),
    }
}

pub fn instant(name: impl Into<cards::CardName>, cost: cards::Cost) -> cards::CardDefinition {
    cards::CardDefinition {
        name: name.into(),
        cost,
        types: vec![cards::CardType::Instant],
        power_toughness: None,
        effect: None,
        keywords: Vec::new(),
    }
}

pub fn sorcery(name: impl Into<cards::CardName>, cost: cards::Cost) -> cards::CardDefinition {
    cards::CardDefinition {
        name: name.into(),
        cost,
        types: vec![cards::CardType::Sorcery],
        power_toughness: None,
        effect: None,
        keywords: Vec::new(),
    }
}

#[must_use]
pub const fn indestructible() -> cards::Keyword {
    cards::Keyword::Indestructible
}

#[must_use]
pub const fn deathtouch() -> cards::Keyword {
    cards::Keyword::Deathtouch
}
