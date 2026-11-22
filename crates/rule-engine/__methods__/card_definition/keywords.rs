use crate::abilities;
use crate::cards;
use crate::keywords;

pub fn keywords(
    definition: &cards::CardDefinition,
) -> impl Iterator<Item = keywords::StaticKeyword> + '_ {
    definition.text.iter().filter_map(|ability| match ability {
        abilities::Ability::Static(abilities::StaticAbility::Keyword(keyword)) => Some(*keyword),
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::cost;
    use crate::keywords;
    use crate::pt;

    use super::keywords as definition_keywords;

    #[test]
    fn test_usage() {
        let definition =
            cards::creature("Grizzly Bears", cost![1, G], pt!(2, 2)).keyword(keywords::flying());
        assert_eq!(
            definition_keywords(&definition).collect::<Vec<_>>(),
            vec![keywords::StaticKeyword::Flying]
        );
    }
}
