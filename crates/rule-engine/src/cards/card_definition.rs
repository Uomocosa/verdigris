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
    pub supertypes: Vec<cards::Supertype>,
    pub types: Vec<cards::CardType>,
    pub land_types: Vec<cards::LandType>,
    pub power_toughness: Option<cards::PowerToughness>,
    pub effect: Option<effects::Effect>,
    pub aura: Option<effects::Effect>,
    pub keywords: Vec<keywords::Keyword>,
    pub triggered_abilities: Vec<abilities::TriggeredAbility>,
    pub does_enter_tapped: bool,
}

impl CardDefinition {
    #[must_use]
    pub fn from_enchantment(name: impl Into<cards::CardName>, cost: cards::Cost) -> Self {
        Self {
            name: name.into(),
            cost,
            supertypes: Vec::new(),
            types: vec![cards::CardType::Enchantment],
            land_types: Vec::new(),
            power_toughness: None,
            effect: None,
            aura: None,
            keywords: Vec::new(),
            triggered_abilities: Vec::new(),
            does_enter_tapped: false,
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
}

#[rustfmt::skip]
impl CardDefinition {
    #[must_use]
    pub fn at(self, trigger: impl Into<abilities::Trigger>, effect: effects::Effect) -> Self { self.whenever(trigger, effect) }

    #[must_use]
    pub fn when(self, trigger: impl Into<abilities::Trigger>, effect: effects::Effect) -> Self { self.whenever(trigger, effect) }

    #[must_use]
    pub fn whenever(self, trigger: impl Into<abilities::Trigger>, effect: effects::Effect) -> Self { self.triggered(abilities::whenever(trigger, effect)) }
}

impl CardDefinition {
    #[must_use]
    pub const fn aura(mut self, effect: effects::Effect) -> Self {
        self.aura = Some(effect);
        self
    }

    #[must_use]
    pub fn keyword(mut self, keyword: keywords::Keyword) -> Self {
        self.keywords.push(keyword);
        self
    }

    #[must_use]
    pub fn supertype(mut self, supertype: cards::Supertype) -> Self {
        self.supertypes.push(supertype);
        self
    }

    #[must_use]
    pub const fn enters_tapped(mut self) -> Self {
        self.does_enter_tapped = true;
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
    ) -> Vec<damage::DamageDealt> {
    }
}

#[cfg(test)]
mod tests {
    use crate::card;
    use crate::cards;
    use crate::cost;
    use crate::effects;
    use crate::keywords;
    use crate::pt;

    #[test]
    fn test_usage() {
        let bears = cards::creature("Grizzly Bears", cost![1, G], pt!(2, 2));
        assert_eq!(bears.types, vec![cards::CardType::Creature]);
        assert_eq!(bears.colors(), vec![cards::Color::Green]);
        let myr = cards::artifact_creature("Darksteel Myr", cost![3], pt!(0, 1))
            .keyword(keywords::indestructible());
        assert_eq!(myr.keywords, vec![keywords::Keyword::Indestructible]);
        let spell = cards::instant("Murder", cards::Cost::default()).spell(effects::destroy());
        assert_eq!(spell.effect, Some(effects::destroy()));
    }

    #[test]
    fn card_macro_lists_one_ability_per_line() {
        let definition = card! {
            cards::creature("Grizzly Bears", cost![1, G], pt!(2, 2));
            keywords::vigilance(),
        };
        assert_eq!(definition.keywords, vec![keywords::Keyword::Vigilance]);
        assert_eq!(definition.triggered_abilities, Vec::new());
    }

    #[test]
    fn card_macro_accepts_zero_abilities() {
        let definition = card! {
            cards::creature("Grizzly Bears", cost![1, G], pt!(2, 2));
        };
        assert_eq!(definition.keywords, Vec::new());
        assert_eq!(definition.triggered_abilities, Vec::new());
    }
}
