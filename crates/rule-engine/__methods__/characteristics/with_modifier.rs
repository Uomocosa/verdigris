use crate::cards;
use crate::effects;

pub const fn with_modifier(
    mut characteristics: cards::Characteristics,
    modifier: Option<&effects::PowerToughnessModifier>,
) -> cards::Characteristics {
    let Some(base) = characteristics.power_toughness else {
        return characteristics;
    };
    let Some(modifier) = modifier else {
        return characteristics;
    };
    characteristics.power_toughness = Some(cards::PowerToughness {
        power: base.power.saturating_add(modifier.power),
        toughness: base.toughness.saturating_add(modifier.toughness),
    });
    characteristics
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::effects;
    use crate::testing;

    use super::with_modifier;

    #[test]
    fn test_usage() {
        let bears = (testing::GRIZZLY_BEARS.definition)();
        let from_printed_value = cards::Characteristics::from_printed(&bears, true);
        assert_eq!(
            with_modifier(from_printed_value.clone(), None).power_toughness,
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
            with_modifier(from_printed_value, Some(&modifier)).power_toughness,
            Some(cards::PowerToughness {
                power: 3,
                toughness: 5
            })
        );
        let spell = cards::instant("Murder", cards::Cost::default());
        assert_eq!(
            with_modifier(
                cards::Characteristics::from_printed(&spell, true),
                Some(&modifier)
            )
            .power_toughness,
            None
        );
    }
}
