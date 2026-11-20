use crate::cards;
use crate::effects;

pub fn effective_power_toughness(
    definition: &cards::CardDefinition,
    modifier: Option<&effects::PowerToughnessModifier>,
) -> Option<cards::PowerToughness> {
    let from_printed_value = cards::Characteristics::from_printed(definition, false);
    from_printed_value.with_modifier(modifier).power_toughness
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::effects;
    use crate::testing;

    use super::effective_power_toughness;

    #[test]
    fn test_usage() {
        let bears = (testing::GRIZZLY_BEARS.definition)();
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
