use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::Component;

use crate::cards;
use crate::effects;
#[cfg(feature = "testing")]
use crate::testing;

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct CardDefinition {
    pub name: cards::CardName,
    pub cost: cards::Cost,
    pub types: Vec<cards::CardType>,
    pub power_toughness: Option<cards::PowerToughness>,
    pub effect: Option<effects::Effect>,
    pub keywords: Vec<cards::Keyword>,
}

#[cfg(feature = "testing")]
#[rustfmt::skip]
impl From<testing::Card> for CardDefinition {
    fn from(card: testing::Card) -> Self { (card.definition)() }
}

#[atomic_delegates]
impl CardDefinition {
    #[must_use]
    pub fn spell(self, effect: effects::Effect) -> Self {}

    #[must_use]
    pub fn colors(&self) -> Vec<cards::Color> {}
}

impl CardDefinition {
    #[must_use]
    pub fn keyword(mut self, keyword: cards::Keyword) -> Self {
        self.keywords.push(keyword);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::cost;
    use crate::effects;
    use crate::pt;

    #[test]
    fn test_usage() {
        let bears = cards::creature("Grizzly Bears", cost![1, G], pt!(2, 2));
        assert_eq!(bears.types, vec![cards::CardType::Creature]);
        assert_eq!(bears.colors(), vec![cards::Color::Green]);
        let myr = cards::artifact_creature("Darksteel Myr", cost![3], pt!(0, 1))
            .keyword(cards::indestructible());
        assert_eq!(myr.keywords, vec![cards::Keyword::Indestructible]);
        let spell = cards::instant("Murder", cards::Cost::default()).spell(effects::destroy());
        assert_eq!(spell.effect, Some(effects::destroy()));
    }
}
