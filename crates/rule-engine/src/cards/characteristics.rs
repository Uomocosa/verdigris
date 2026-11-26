use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::{Component, EntityCommands};

use crate::abilities;
use crate::cards;
use crate::effects;
use crate::keywords;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Characteristics {
    pub on_battlefield: bool,
    pub types: Vec<cards::CardType>,
    pub power_toughness: Option<cards::PowerToughness>,
    pub keywords: Vec<keywords::Keyword>,
    pub triggered_abilities: Vec<abilities::TriggeredAbility>,
}

#[atomic_delegates]
impl Characteristics {
    #[must_use]
    pub fn from_printed(definition: &cards::CardDefinition, on_battlefield: bool) -> Self {}
}

#[atomic_delegates]
impl Characteristics {
    #[must_use]
    pub fn with_effects(self, effects: &[effects::ContinuousEffect]) -> Self {}

    pub fn write_markers(
        &self,
        entity: &mut EntityCommands,
        current: cards::CurrentMarkersItem<'_, '_, '_>,
    ) {
    }
}

#[atomic_delegates]
impl Characteristics {
    pub(crate) fn write_triggered_markers(
        &self,
        entity: &mut EntityCommands,
        current: cards::CurrentMarkersItem<'_, '_, '_>,
    ) {
    }
}

impl Characteristics {
    pub(crate) fn sync_marker<M>(
        entity: &mut EntityCommands,
        desired: bool,
        current: bool,
        marker: M,
    ) where
        M: Component,
    {
        if desired == current {
            return;
        }
        if desired {
            entity.insert(marker);
        } else {
            entity.remove::<M>();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::cost;
    use crate::pt;

    use super::Characteristics;

    #[test]
    fn test_usage() {
        let bears = cards::creature("Grizzly Bears", cost![1, G], pt!(2, 2));
        let characteristics = Characteristics::from_printed(&bears, true);
        assert!(characteristics.on_battlefield);
        assert_eq!(characteristics.types, vec![cards::CardType::Creature]);
        assert_eq!(characteristics.power_toughness, Some(pt!(2, 2)));
        assert_eq!(characteristics.keywords, Vec::new());
        assert_eq!(characteristics.triggered_abilities, Vec::new());
    }
}
