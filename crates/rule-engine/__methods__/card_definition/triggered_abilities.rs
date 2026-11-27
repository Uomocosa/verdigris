use crate::abilities;
use crate::cards;

pub fn triggered_abilities(
    definition: &cards::CardDefinition,
) -> impl Iterator<Item = &abilities::TriggeredAbility> {
    definition.text.iter().filter_map(|ability| match ability {
        abilities::Ability::Triggered(triggered) => Some(triggered),
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use crate::YOU;
    use crate::abilities;
    use crate::another_creature;
    use crate::cards;
    use crate::cost;
    use crate::pt;

    use super::triggered_abilities;

    #[test]
    fn test_usage() {
        let ability = abilities::whenever(another_creature().enters(), YOU.gain_life(1));
        let definition =
            cards::creature("Grizzly Bears", cost![1, G], pt!(2, 2)).triggered(ability.clone());
        assert_eq!(
            triggered_abilities(&definition).cloned().collect::<Vec<_>>(),
            [ability]
        );
    }
}
