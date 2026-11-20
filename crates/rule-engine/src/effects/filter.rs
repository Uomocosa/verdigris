use atomic_delegate_macros::atomic_delegates;

use crate::cards;
use crate::effects;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Filter {
    pub kind: effects::FilterKind,
    pub controlled_by: Option<effects::PlayerRef>,
    pub excluded_color: Option<cards::Color>,
}

#[atomic_delegates]
impl Filter {
    #[must_use]
    pub fn controlled_by(self, player: effects::PlayerRef) -> Self {}
}

impl Filter {
    #[must_use]
    pub fn does_creature_match(self, definition: &cards::CardDefinition) -> bool {
        let is_creature = definition.types.contains(&cards::CardType::Creature);
        let has_excluded_color =
            self.excluded_color.is_some_and(|excluded| definition.colors().contains(&excluded));
        is_creature && !has_excluded_color
    }

    #[must_use]
    pub fn does_spell_match(self, definition: &cards::CardDefinition) -> bool {
        let is_creature = definition.types.contains(&cards::CardType::Creature);
        let is_instant = definition.types.contains(&cards::CardType::Instant);
        match self.kind {
            effects::FilterKind::Spell => true,
            effects::FilterKind::CreatureSpell => is_creature,
            effects::FilterKind::NoncreatureSpell => !is_creature,
            effects::FilterKind::InstantSpell => is_instant,
            effects::FilterKind::Creature
            | effects::FilterKind::Player
            | effects::FilterKind::AnyTarget => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ANY_TARGET;
    use crate::CREATURE;
    use crate::cards;
    use crate::effects;
    use crate::{cost, pt};

    #[test]
    fn test_usage() {
        assert_eq!(CREATURE.kind, effects::FilterKind::Creature);
        assert_eq!(CREATURE.excluded_color, None);
        assert_eq!(ANY_TARGET.kind, effects::FilterKind::AnyTarget);
        assert_eq!(
            CREATURE.controlled_by(effects::PlayerRef::You).controlled_by,
            Some(effects::PlayerRef::You)
        );
        let bears = cards::creature("Grizzly Bears", cost![1, G], pt!(2, 2));
        let bolt = cards::instant("Lightning Bolt", cards::Cost::default());
        let instant_spell = effects::Filter {
            kind: effects::FilterKind::InstantSpell,
            ..ANY_TARGET
        };
        assert!(CREATURE.does_creature_match(&bears));
        assert!(!CREATURE.does_spell_match(&bears));
        assert!(instant_spell.does_spell_match(&bolt));
        assert!(!instant_spell.does_creature_match(&bolt));
    }
}
