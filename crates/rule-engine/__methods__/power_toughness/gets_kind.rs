use crate::cards;
use crate::effects;

pub const fn gets_kind(power_toughness: cards::PowerToughness) -> effects::EffectKind {
    effects::EffectKind::ModifyPowerToughness {
        power: power_toughness.power,
        toughness: power_toughness.toughness,
    }
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::effects;

    use super::gets_kind;

    #[test]
    fn test_usage() {
        let boost = cards::PowerToughness {
            power: 2,
            toughness: 1,
        };
        assert_eq!(
            gets_kind(boost),
            effects::EffectKind::ModifyPowerToughness {
                power: 2,
                toughness: 1
            }
        );
    }
}
