use crate::cards;
use crate::effects;

pub fn with_effects(
    mut characteristics: cards::Characteristics,
    effects: &[effects::ContinuousEffect],
) -> cards::Characteristics {
    for effect in effects {
        if let effects::Layered::GainKeyword(keyword) = effect.change
            && !characteristics.keywords.contains(&keyword)
        {
            characteristics.keywords.push(keyword);
        }
    }
    let Some(base) = characteristics.power_toughness else {
        return characteristics;
    };
    let mut power = base.power;
    let mut toughness = base.toughness;
    for effect in effects {
        if let effects::Layered::ModifyPT {
            power: delta_power,
            toughness: delta_toughness,
        } = effect.change
        {
            power = power.saturating_add(delta_power);
            toughness = toughness.saturating_add(delta_toughness);
        }
    }
    characteristics.power_toughness = Some(cards::PowerToughness { power, toughness });
    characteristics
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::cards;
    use crate::effects;
    use crate::testing;

    use super::with_effects;

    #[test]
    fn test_usage() {
        let bears = (testing::GRIZZLY_BEARS.definition)();
        let from_printed_value = cards::Characteristics::from_printed(&bears, true);
        assert_eq!(
            with_effects(from_printed_value.clone(), &[]).power_toughness,
            Some(cards::PowerToughness {
                power: 2,
                toughness: 2
            })
        );
        let effect = effects::ContinuousEffect {
            affected: Entity::PLACEHOLDER,
            change: effects::Layered::ModifyPT {
                power: 1,
                toughness: 3,
            },
            until: effects::Duration::UntilEndOfTurn,
            timestamp: 0,
        };
        assert_eq!(
            with_effects(from_printed_value.clone(), &[effect]).power_toughness,
            Some(cards::PowerToughness {
                power: 3,
                toughness: 5
            })
        );
        let spell = cards::instant("Murder", cards::Cost::default());
        assert_eq!(
            with_effects(
                cards::Characteristics::from_printed(&spell, true),
                &[effect]
            )
            .power_toughness,
            None
        );
        let grant = effects::ContinuousEffect {
            affected: Entity::PLACEHOLDER,
            change: effects::Layered::GainKeyword(cards::Keyword::Flying),
            until: effects::Duration::UntilEndOfTurn,
            timestamp: 0,
        };
        let granted = with_effects(from_printed_value, &[grant]);
        assert!(granted.keywords.contains(&cards::Keyword::Flying));
        assert_eq!(
            granted.power_toughness,
            Some(cards::PowerToughness {
                power: 2,
                toughness: 2
            })
        );
    }
}
