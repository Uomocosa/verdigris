use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;

use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct DefinitionId(pub &'static str);

pub const LETHAL_DAMAGE: DefinitionId = DefinitionId("glossary/lethal-damage");

#[atomic_delegates]
impl DefinitionId {
    #[must_use]
    pub fn definition(
        self,
        text: &'static str,
        role: rules::Role,
        condition: rules::Condition,
    ) -> rules::Definition {
    }
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::rules;

    use super::DefinitionId;
    use super::LETHAL_DAMAGE;

    #[test]
    fn test_usage() {
        let id = DefinitionId("glossary/lethal-damage");
        assert_eq!(LETHAL_DAMAGE, DefinitionId("glossary/lethal-damage"));
        let built = id.definition(
            "An amount of damage greater than or equal to a creature's toughness.",
            rules::Role::Other,
            !rules::has::<cards::Tapped>(),
        );
        assert_eq!(built.id, DefinitionId("glossary/lethal-damage"));
        assert_eq!(
            built.text,
            "An amount of damage greater than or equal to a creature's toughness."
        );
        assert_eq!(built.role, rules::Role::Other);
        assert_eq!(built.condition, !rules::has::<cards::Tapped>());
    }
}
