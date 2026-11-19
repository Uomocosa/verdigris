use crate::cards;
use crate::effects;

pub fn spell(definition: cards::CardDefinition, effect: effects::Effect) -> cards::CardDefinition {
    cards::CardDefinition {
        effect: Some(effect),
        ..definition
    }
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::effects;

    use super::spell;

    #[test]
    fn test_usage() {
        let definition = cards::instant("Murder", cards::Cost::default());
        let spell = spell(definition, effects::destroy());
        assert_eq!(spell.effect, Some(effects::destroy()));
    }
}
