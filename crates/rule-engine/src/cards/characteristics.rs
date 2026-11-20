use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::EntityCommands;

use crate::cards;
use crate::effects;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Characteristics {
    pub on_battlefield: bool,
    pub power_toughness: Option<cards::PowerToughness>,
    pub keywords: Vec<cards::Keyword>,
}

#[atomic_delegates]
impl Characteristics {
    #[must_use]
    pub fn from_printed(definition: &cards::CardDefinition, on_battlefield: bool) -> Self {}
}

#[atomic_delegates]
impl Characteristics {
    #[must_use]
    pub const fn with_modifier(self, modifier: Option<&effects::PowerToughnessModifier>) -> Self {}

    pub fn write_markers(
        &self,
        entity: &mut EntityCommands,
        current: (
            bool,
            Option<cards::PowerToughness>,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
        ),
    ) {
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
        assert_eq!(characteristics.power_toughness, Some(pt!(2, 2)));
        assert_eq!(characteristics.keywords, Vec::new());
    }
}
