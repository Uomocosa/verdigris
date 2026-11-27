use crate::abilities;
use crate::cards;
use crate::effects;

pub fn spell_ability(definition: &cards::CardDefinition) -> Option<&effects::Effect> {
    definition.text.iter().find_map(|ability| match ability {
        abilities::Ability::Spell(effect) => Some(effect),
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::cards;
    use crate::cost;
    use crate::effects;

    use super::spell_ability;

    #[test]
    fn test_usage() {
        let definition = cards::instant("Murder", cost![1, B, B])
            .spell(effects::destroy(effects::target(1, CREATURE)));
        assert_eq!(
            spell_ability(&definition),
            Some(&effects::destroy(effects::target(1, CREATURE)))
        );
    }
}
