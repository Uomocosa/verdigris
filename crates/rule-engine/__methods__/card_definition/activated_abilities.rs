use crate::abilities;
use crate::cards;

pub fn activated_abilities(
    definition: &cards::CardDefinition,
) -> impl Iterator<Item = &abilities::ActivatedAbility> {
    definition.text.iter().filter_map(|ability| match ability {
        abilities::Ability::Activated(activated) => Some(activated),
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use crate::abilities;
    use crate::card;
    use crate::cards;
    use crate::cost;
    use crate::pt;

    use super::activated_abilities;

    #[test]
    fn test_usage() {
        let ability = abilities::tap_add(cards::ManaType::Colorless);
        let definition = card! {
            cards::land("Reliquary Tower", []);
            ability,
        };
        assert_eq!(
            activated_abilities(&definition).copied().collect::<Vec<_>>(),
            [ability]
        );
        let bears = cards::creature("Grizzly Bears", cost![1, G], pt!(2, 2));
        assert_eq!(activated_abilities(&bears).count(), 0);
    }
}
