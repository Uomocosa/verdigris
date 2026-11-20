use crate::cards;
use crate::effects;

pub fn effective_power_toughness(
    definition: &cards::CardDefinition,
    modifier: Option<&effects::PowerToughnessModifier>,
) -> Option<cards::PowerToughness> {
    let base = definition.power_toughness?;
    let Some(modifier) = modifier else {
        return Some(base);
    };
    Some(cards::PowerToughness {
        power: base.power.saturating_add(modifier.power),
        toughness: base.toughness.saturating_add(modifier.toughness),
    })
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::effects;
    use crate::{cost, pt};

    use super::effective_power_toughness;

    #[test]
    fn test_usage() {
        let bears = cards::creature("Grizzly Bears", cost![1, G], pt!(2, 2));
        assert_eq!(
            effective_power_toughness(&bears, None),
            Some(cards::PowerToughness {
                power: 2,
                toughness: 2
            })
        );
        let modifier = effects::PowerToughnessModifier {
            power: 1,
            toughness: 3,
        };
        assert_eq!(
            effective_power_toughness(&bears, Some(&modifier)),
            Some(cards::PowerToughness {
                power: 3,
                toughness: 5
            })
        );
    }
}
