use crate::abilities;
use crate::cards;

pub fn enters_tapped(definition: &cards::CardDefinition) -> bool {
    definition.text.iter().any(|ability| {
        matches!(
            ability,
            abilities::Ability::Static(abilities::StaticAbility::Replacement(
                abilities::Replacement::EntersTapped
            ))
        )
    })
}

#[cfg(test)]
mod tests {
    use crate::cards;

    use super::enters_tapped;

    #[test]
    fn test_usage() {
        let definition = cards::land("Gruul Guildgate", []).with_enters_tapped();
        assert!(enters_tapped(&definition));
    }
}
