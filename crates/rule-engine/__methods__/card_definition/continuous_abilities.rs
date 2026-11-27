use crate::abilities;
use crate::cards;

pub fn continuous_abilities(
    definition: &cards::CardDefinition,
) -> impl Iterator<Item = &abilities::Continuous> {
    definition.text.iter().filter_map(|ability| match ability {
        abilities::Ability::Static(abilities::StaticAbility::Continuous(continuous)) => {
            Some(continuous)
        }
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use crate::abilities;
    use crate::cards;
    use crate::cost;
    use crate::creature;
    use crate::keywords;
    use crate::pt;
    use crate::statics;

    use super::continuous_abilities;

    #[test]
    fn test_usage() {
        let continuous = statics::all(creature()).gain(keywords::vigilance());
        let definition = cards::creature("Grizzly Bears", cost![1, G], pt!(2, 2))
            .abilities([abilities::Ability::from(continuous.clone())]);
        assert_eq!(
            continuous_abilities(&definition).cloned().collect::<Vec<_>>(),
            [continuous]
        );
    }
}
