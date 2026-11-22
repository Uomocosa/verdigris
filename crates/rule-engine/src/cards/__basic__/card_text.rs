use crate::cards;

pub fn creature(
    name: impl Into<cards::CardName>,
    cost: cards::Cost,
    power_toughness: cards::PowerToughness,
) -> cards::CardDefinition {
    cards::CardDefinition {
        name: name.into(),
        cost,
        type_line: cards::TypeLine {
            supertypes: Vec::new(),
            types: vec![cards::CardType::Creature],
            subtypes: Vec::new(),
        },
        power_toughness: Some(power_toughness),
        effect: None,
        aura: None,
        triggered_abilities: Vec::new(),
        static_abilities: Vec::new(),
        keywords: Vec::new(),
        does_enter_tapped: false,
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
        type_line: cards::TypeLine {
            supertypes: Vec::new(),
            types: vec![cards::CardType::Artifact, cards::CardType::Creature],
            subtypes: Vec::new(),
        },
        power_toughness: Some(power_toughness),
        effect: None,
        aura: None,
        triggered_abilities: Vec::new(),
        static_abilities: Vec::new(),
        keywords: Vec::new(),
        does_enter_tapped: false,
    }
}

pub fn instant(name: impl Into<cards::CardName>, cost: cards::Cost) -> cards::CardDefinition {
    cards::CardDefinition {
        name: name.into(),
        cost,
        type_line: cards::TypeLine {
            supertypes: Vec::new(),
            types: vec![cards::CardType::Instant],
            subtypes: Vec::new(),
        },
        power_toughness: None,
        effect: None,
        aura: None,
        triggered_abilities: Vec::new(),
        static_abilities: Vec::new(),
        keywords: Vec::new(),
        does_enter_tapped: false,
    }
}

pub fn sorcery(name: impl Into<cards::CardName>, cost: cards::Cost) -> cards::CardDefinition {
    cards::CardDefinition {
        name: name.into(),
        cost,
        type_line: cards::TypeLine {
            supertypes: Vec::new(),
            types: vec![cards::CardType::Sorcery],
            subtypes: Vec::new(),
        },
        power_toughness: None,
        effect: None,
        aura: None,
        triggered_abilities: Vec::new(),
        static_abilities: Vec::new(),
        keywords: Vec::new(),
        does_enter_tapped: false,
    }
}

pub fn land(
    name: impl Into<cards::CardName>,
    land_subtypes: impl IntoIterator<Item = cards::LandType>,
) -> cards::CardDefinition {
    cards::CardDefinition {
        name: name.into(),
        cost: cards::Cost::default(),
        type_line: cards::TypeLine {
            supertypes: Vec::new(),
            types: vec![cards::CardType::Land],
            subtypes: land_subtypes.into_iter().map(cards::Subtype::from).collect(),
        },
        power_toughness: None,
        effect: None,
        aura: None,
        triggered_abilities: Vec::new(),
        static_abilities: Vec::new(),
        keywords: Vec::new(),
        does_enter_tapped: false,
    }
}
