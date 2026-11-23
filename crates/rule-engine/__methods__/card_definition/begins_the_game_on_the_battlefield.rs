use crate::abilities;
use crate::cards;

pub fn begins_the_game_on_the_battlefield(definition: &cards::CardDefinition) -> bool {
    definition.text.iter().any(|ability| {
        matches!(
            ability,
            abilities::Ability::Static(abilities::StaticAbility::OpeningHand)
        )
    })
}

#[cfg(test)]
mod tests {
    use crate::abilities;
    use crate::cards;
    use crate::cost;
    use crate::pt;

    use super::begins_the_game_on_the_battlefield;

    #[test]
    fn test_usage() {
        let leyline =
            cards::CardDefinition::from_enchantment("Leyline", cost![2, U, U]).abilities([
                abilities::Ability::from(abilities::begin_the_game_with_it_on_the_battlefield()),
            ]);
        assert!(begins_the_game_on_the_battlefield(&leyline));
        let bears = cards::creature("Grizzly Bears", cost![1, G], pt!(2, 2));
        assert!(!begins_the_game_on_the_battlefield(&bears));
    }
}
