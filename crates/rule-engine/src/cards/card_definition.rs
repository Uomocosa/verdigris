use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::Component;

use crate::cards;
use crate::effects;

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct CardDefinition {
    pub name: cards::CardName,
    pub cost: cards::Cost,
    pub types: Vec<cards::CardType>,
    pub power_toughness: Option<cards::PowerToughness>,
    pub effect: Option<effects::Effect>,
}

#[atomic_delegates]
impl CardDefinition {
    #[must_use]
    pub fn spell(self, effect: effects::Effect) -> Self {}
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::effects;
    use crate::pt;

    #[test]
    fn test_usage() {
        let bears = cards::creature("Grizzly Bears", cards::Cost::default(), pt!(2, 2));
        assert_eq!(bears.types, vec![cards::CardType::Creature]);
        let spell = cards::instant("Murder", cards::Cost::default()).spell(effects::destroy());
        assert_eq!(spell.effect, Some(effects::destroy()));
    }
}
