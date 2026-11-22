use crate::abilities;
use crate::cards;
use crate::effects;

pub fn spell(
    mut definition: cards::CardDefinition,
    effect: effects::Effect,
) -> cards::CardDefinition {
    definition.text.push(abilities::Ability::Spell(effect));
    definition
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::cost;
    use crate::effects;

    use super::spell;

    #[test]
    fn test_usage() {
        let definition = cards::instant("Murder", cost![1, B, B]);
        let spell = spell(definition, effects::destroy());
        assert_eq!(spell.spell_ability(), Some(&effects::destroy()));
    }
}
