use atomic_delegate_macros::atomic_delegates;

use crate::cards;
use crate::effects;
use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Filter {
    pub kind: effects::FilterKind,
    pub controlled_by: Option<rules::Players>,
    pub excluded_color: Option<cards::Color>,
    pub attacking: bool,
    pub nontoken: bool,
    pub object: Option<effects::ObjectRelation>,
}

#[rustfmt::skip]
impl From<rules::Players> for Filter {
    fn from(players: rules::Players) -> Self { Self::player(players) }
}

#[rustfmt::skip]
impl From<rules::You> for Filter {
    fn from(you: rules::You) -> Self { Self::player(you) }
}

#[atomic_delegates]
impl Filter {
    #[must_use]
    pub fn player(who: impl Into<rules::Players>) -> Self {}
}

#[atomic_delegates]
impl Filter {
    #[must_use]
    pub fn controlled_by(self, player: impl Into<rules::Players>) -> Self {}

    #[must_use]
    pub fn attacking(self) -> Self {}

    #[must_use]
    pub fn nontoken(self) -> Self {}
}

impl Filter {
    #[must_use]
    pub fn does_creature_match(self, definition: &cards::CardDefinition) -> bool {
        let is_creature = definition.type_line.types.contains(&cards::CardType::Creature);
        let has_excluded_color =
            self.excluded_color.is_some_and(|excluded| definition.colors().contains(&excluded));
        is_creature && !has_excluded_color
    }

    #[must_use]
    pub fn does_enchantment_match(self, definition: &cards::CardDefinition) -> bool {
        let is_enchantment = definition.type_line.types.contains(&cards::CardType::Enchantment);
        let has_excluded_color =
            self.excluded_color.is_some_and(|excluded| definition.colors().contains(&excluded));
        is_enchantment && !has_excluded_color
    }

    #[must_use]
    pub fn does_spell_match(self, definition: &cards::CardDefinition) -> bool {
        let is_creature = definition.type_line.types.contains(&cards::CardType::Creature);
        let is_instant = definition.type_line.types.contains(&cards::CardType::Instant);
        match self.kind {
            effects::FilterKind::Spell => true,
            effects::FilterKind::CreatureSpell => is_creature,
            effects::FilterKind::NoncreatureSpell => !is_creature,
            effects::FilterKind::InstantSpell => is_instant,
            effects::FilterKind::Creature
            | effects::FilterKind::Enchantment
            | effects::FilterKind::Player
            | effects::FilterKind::AnyTarget => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ANY_TARGET;
    use crate::CREATURE;
    use crate::effects;
    use crate::rules;
    use crate::testing;

    #[test]
    fn test_usage() {
        assert_eq!(CREATURE.kind, effects::FilterKind::Creature);
        assert_eq!(CREATURE.excluded_color, None);
        assert_eq!(ANY_TARGET.kind, effects::FilterKind::AnyTarget);
        assert_eq!(
            CREATURE.controlled_by(crate::YOU).controlled_by,
            Some(rules::Players::You)
        );
        let bears = (testing::GRIZZLY_BEARS.definition)();
        let bolt = (testing::LIGHTNING_BOLT.definition)();
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
