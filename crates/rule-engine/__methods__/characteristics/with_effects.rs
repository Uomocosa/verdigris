use crate::cards;
use crate::effects;

pub fn with_effects(
    mut characteristics: cards::Characteristics,
    effects: &[effects::ContinuousEffect],
) -> cards::Characteristics {
    for effect in effects {
        if let effects::Change::Characteristic(change) = effect.change {
            if let effects::Layered::GainKeyword(keyword) = change
                && !characteristics.keywords.contains(&keyword)
            {
                characteristics.keywords.push(keyword);
            }
            if let effects::Layered::LoseKeyword(keyword) = change {
                characteristics.keywords.retain(|kept| *kept != keyword);
            }
        }
    }
    let Some(base) = characteristics.power_toughness else {
        return characteristics;
    };
    let mut power = base.power;
    let mut toughness = base.toughness;
    for effect in effects {
        if let effects::Change::Characteristic(effects::Layered::ModifyPT {
            power: delta_power,
            toughness: delta_toughness,
        }) = effect.change
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
    use crate::keywords;
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
            source: None,
            change: effects::Change::Characteristic(effects::Layered::ModifyPT {
                power: 1,
                toughness: 3,
            }),
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
            source: None,
            change: effects::Change::Characteristic(effects::Layered::GainKeyword(
                keywords::StaticKeyword::Flying,
            )),
            until: effects::Duration::UntilEndOfTurn,
            timestamp: 0,
        };
        let granted = with_effects(from_printed_value, &[grant]);
        assert!(granted.keywords.contains(&keywords::StaticKeyword::Flying));
        assert_eq!(
            granted.power_toughness,
            Some(cards::PowerToughness {
                power: 2,
                toughness: 2
            })
        );
    }

    #[test]
    fn losing_a_keyword_removes_it_from_characteristics() {
        let bears = (testing::GRIZZLY_BEARS.definition)();
        let from_printed_value = cards::Characteristics::from_printed(&bears, true);
        let gain = effects::ContinuousEffect {
            affected: Entity::PLACEHOLDER,
            source: None,
            change: effects::Change::Characteristic(effects::Layered::GainKeyword(
                keywords::StaticKeyword::Flying,
            )),
            until: effects::Duration::UntilEndOfTurn,
            timestamp: 0,
        };
        let lose = effects::ContinuousEffect {
            affected: Entity::PLACEHOLDER,
            source: None,
            change: effects::Change::Characteristic(effects::Layered::LoseKeyword(
                keywords::StaticKeyword::Flying,
            )),
            until: effects::Duration::UntilEndOfTurn,
            timestamp: 1,
        };
        let removed = with_effects(from_printed_value, &[gain, lose]);
        assert!(!removed.keywords.contains(&keywords::StaticKeyword::Flying));
        assert_eq!(
            removed.power_toughness,
            Some(cards::PowerToughness {
                power: 2,
                toughness: 2
            })
        );
    }

    #[test]
    fn a_later_gain_reapplies_a_keyword_after_a_loss() {
        let bears = (testing::GRIZZLY_BEARS.definition)();
        let from_printed_value = cards::Characteristics::from_printed(&bears, true);
        let lose = effects::ContinuousEffect {
            affected: Entity::PLACEHOLDER,
            source: None,
            change: effects::Change::Characteristic(effects::Layered::LoseKeyword(
                keywords::StaticKeyword::Flying,
            )),
            until: effects::Duration::UntilEndOfTurn,
            timestamp: 0,
        };
        let gain = effects::ContinuousEffect {
            affected: Entity::PLACEHOLDER,
            source: None,
            change: effects::Change::Characteristic(effects::Layered::GainKeyword(
                keywords::StaticKeyword::Flying,
            )),
            until: effects::Duration::UntilEndOfTurn,
            timestamp: 1,
        };
        let regained = with_effects(from_printed_value, &[lose, gain]);
        assert!(regained.keywords.contains(&keywords::StaticKeyword::Flying));
    }
}
