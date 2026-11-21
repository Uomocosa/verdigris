use crate::cards;
use crate::effects;
use crate::turns;

pub fn creature(
    name: impl Into<cards::CardName>,
    cost: cards::Cost,
    power_toughness: cards::PowerToughness,
) -> cards::CardDefinition {
    cards::CardDefinition {
        name: name.into(),
        cost,
        supertypes: Vec::new(),
        types: vec![cards::CardType::Creature],
        land_types: Vec::new(),
        power_toughness: Some(power_toughness),
        effect: None,
        triggered_abilities: Vec::new(),
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
        supertypes: Vec::new(),
        types: vec![cards::CardType::Artifact, cards::CardType::Creature],
        land_types: Vec::new(),
        power_toughness: Some(power_toughness),
        effect: None,
        triggered_abilities: Vec::new(),
        keywords: Vec::new(),
    }
}

pub fn instant(name: impl Into<cards::CardName>, cost: cards::Cost) -> cards::CardDefinition {
    cards::CardDefinition {
        name: name.into(),
        cost,
        supertypes: Vec::new(),
        types: vec![cards::CardType::Instant],
        land_types: Vec::new(),
        power_toughness: None,
        effect: None,
        triggered_abilities: Vec::new(),
        keywords: Vec::new(),
    }
}

pub fn sorcery(name: impl Into<cards::CardName>, cost: cards::Cost) -> cards::CardDefinition {
    cards::CardDefinition {
        name: name.into(),
        cost,
        supertypes: Vec::new(),
        types: vec![cards::CardType::Sorcery],
        land_types: Vec::new(),
        power_toughness: None,
        effect: None,
        triggered_abilities: Vec::new(),
        keywords: Vec::new(),
    }
}

pub fn land(
    name: impl Into<cards::CardName>,
    land_types: impl IntoIterator<Item = cards::LandType>,
) -> cards::CardDefinition {
    cards::CardDefinition {
        name: name.into(),
        cost: cards::Cost::default(),
        supertypes: Vec::new(),
        types: vec![cards::CardType::Land],
        land_types: land_types.into_iter().collect(),
        power_toughness: None,
        effect: None,
        triggered_abilities: Vec::new(),
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

#[must_use]
pub const fn lifelink() -> cards::Keyword {
    cards::Keyword::Lifelink
}

#[must_use]
pub const fn trample() -> cards::Keyword {
    cards::Keyword::Trample
}

#[must_use]
pub const fn flying() -> cards::Keyword {
    cards::Keyword::Flying
}

#[must_use]
pub const fn reach() -> cards::Keyword {
    cards::Keyword::Reach
}

#[must_use]
pub const fn vigilance() -> cards::Keyword {
    cards::Keyword::Vigilance
}

#[must_use]
pub const fn beginning_of(step: turns::Step, player: effects::PlayerRef) -> cards::TriggerEvent {
    cards::TriggerEvent::BeginningOfStep(step, player)
}
