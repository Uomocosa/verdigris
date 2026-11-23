use std::collections::BTreeSet;

use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::{Component, Entity, Query};

use crate::abilities;
use crate::actions;
use crate::cards;
use crate::damage;
use crate::effects;
use crate::keywords;
#[cfg(feature = "testing")]
use crate::testing;

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct CardDefinition {
    pub name: cards::CardName,
    pub cost: cards::Cost,
    pub type_line: cards::TypeLine,
    pub power_toughness: Option<cards::PowerToughness>,
    pub text: Vec<abilities::Ability>,
}

impl CardDefinition {
    #[must_use]
    pub fn from_artifact(name: impl Into<cards::CardName>, cost: cards::Cost) -> Self {
        Self {
            name: name.into(),
            cost,
            type_line: cards::TypeLine {
                supertypes: Vec::new(),
                types: vec![cards::CardType::Artifact],
                subtypes: Vec::new(),
            },
            power_toughness: None,
            text: Vec::new(),
        }
    }

    #[must_use]
    pub fn from_enchantment(name: impl Into<cards::CardName>, cost: cards::Cost) -> Self {
        Self {
            name: name.into(),
            cost,
            type_line: cards::TypeLine {
                supertypes: Vec::new(),
                types: vec![cards::CardType::Enchantment],
                subtypes: Vec::new(),
            },
            power_toughness: None,
            text: Vec::new(),
        }
    }
}

#[cfg(feature = "testing")]
#[rustfmt::skip]
impl From<testing::Card> for CardDefinition {
    fn from(card: testing::Card) -> Self { (card.definition)() }
}

#[atomic_delegates]
impl CardDefinition {
    #[must_use]
    pub fn abilities<const N: usize>(self, list: [abilities::Ability; N]) -> Self {}

    #[must_use]
    pub fn spell(self, effect: effects::Effect) -> Self {}

    #[must_use]
    pub fn triggered(self, ability: abilities::TriggeredAbility) -> Self {}

    #[must_use]
    pub fn colors(&self) -> Vec<cards::Color> {}

    #[must_use]
    pub fn spell_ability(&self) -> Option<&effects::Effect> {}

    pub fn triggered_abilities(&self) -> impl Iterator<Item = &abilities::TriggeredAbility> {}

    pub fn activated_abilities(&self) -> impl Iterator<Item = &abilities::ActivatedAbility> {}

    pub fn continuous_abilities(&self) -> impl Iterator<Item = &abilities::Continuous> {}

    pub fn keywords(&self) -> impl Iterator<Item = keywords::StaticKeyword> + '_ {}

    #[must_use]
    pub fn enters_tapped(&self) -> bool {}

    #[must_use]
    pub fn begins_the_game_on_the_battlefield(&self) -> bool {}

    #[must_use]
    pub fn enchant(&self) -> Option<effects::Filter> {}
}

#[rustfmt::skip]
impl CardDefinition {
    #[must_use]
    pub fn has_keyword(&self, keyword: keywords::StaticKeyword) -> bool { self.keywords().any(|candidate| candidate == keyword) }

    #[must_use]
    pub fn at(self, trigger: impl Into<abilities::Trigger>, effect: effects::Effect) -> Self { self.whenever(trigger, effect) }

    #[must_use]
    pub fn when(self, trigger: impl Into<abilities::Trigger>, effect: effects::Effect) -> Self { self.whenever(trigger, effect) }

    #[must_use]
    pub fn whenever(self, trigger: impl Into<abilities::Trigger>, effect: effects::Effect) -> Self { self.triggered(abilities::whenever(trigger, effect)) }
}

impl CardDefinition {
    #[must_use]
    pub fn keyword(mut self, keyword: keywords::StaticKeyword) -> Self {
        self.text.push(abilities::Ability::from(keyword));
        self
    }

    #[must_use]
    pub fn supertype(mut self, supertype: cards::Supertype) -> Self {
        self.type_line.supertypes.push(supertype);
        self
    }

    #[must_use]
    pub fn subtype(mut self, subtype: cards::Subtype) -> Self {
        self.type_line.subtypes.push(subtype);
        self
    }

    #[must_use]
    pub fn with_enters_tapped(mut self) -> Self {
        self.text.push(abilities::Ability::from(
            abilities::Replacement::EntersTapped,
        ));
        self
    }
}

#[atomic_delegates]
impl CardDefinition {
    #[must_use]
    pub(crate) fn assigned_blocked_damage(
        &self,
        source: Entity,
        amount: u32,
        defending: Entity,
        blockers: &Query<(
            Entity,
            &cards::Blocking,
            Option<&damage::Damage>,
            Option<&cards::EffectivePT>,
        )>,
        participants: &BTreeSet<Entity>,
        needed: &mut Vec<actions::RequiredCombatDamage>,
        source_has_deathtouch: bool,
    ) -> Vec<damage::AssignedDamage> {
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::card;
    use crate::cards;
    use crate::cost;
    use crate::effects;
    use crate::keywords;
    use crate::pt;
    use crate::statics;

    #[test]
    fn test_usage() {
        let bears = cards::creature("Grizzly Bears", cost![1, G], pt!(2, 2));
        assert_eq!(bears.type_line.types, vec![cards::CardType::Creature]);
        assert_eq!(bears.colors(), vec![cards::Color::Green]);
        let myr = cards::artifact_creature("Darksteel Myr", cost![3], pt!(0, 1))
            .keyword(keywords::indestructible());
        assert_eq!(
            myr.keywords().collect::<Vec<_>>(),
            vec![keywords::StaticKeyword::Indestructible]
        );
        let spell = cards::instant("Murder", cards::Cost::default()).spell(effects::destroy());
        assert_eq!(spell.spell_ability(), Some(&effects::destroy()));
    }

    #[test]
    fn card_macro_lists_one_ability_per_line() {
        let definition = card! {
            cards::creature("Grizzly Bears", cost![1, G], pt!(2, 2));
            keywords::vigilance(),
            statics::all(CREATURE).gain(keywords::vigilance()),
        };
        assert_eq!(
            definition.keywords().collect::<Vec<_>>(),
            vec![keywords::StaticKeyword::Vigilance]
        );
        assert_eq!(definition.triggered_abilities().count(), 0);
        let continuous = definition.continuous_abilities().collect::<Vec<_>>();
        assert_eq!(continuous.len(), 1);
        assert_eq!(
            continuous[0].modification,
            effects::Modification::Characteristic(effects::Layered::GainKeyword(
                keywords::StaticKeyword::Vigilance
            ))
        );
    }

    #[test]
    fn card_macro_accepts_zero_abilities() {
        let definition = card! {
            cards::creature("Grizzly Bears", cost![1, G], pt!(2, 2));
        };
        assert_eq!(definition.keywords().count(), 0);
        assert_eq!(definition.triggered_abilities().count(), 0);
    }

    #[test]
    fn card_macro_accepts_an_enters_tapped_replacement() {
        let definition = card! {
            cards::land("Gruul Guildgate", []);
            statics::enters_tapped(),
        };
        assert!(definition.enters_tapped());
    }
}
