use atomic_delegate_macros::atomic_delegate;

use crate::effects;
use crate::effects::traits::gets::Gets;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PowerToughness {
    pub power: i32,
    pub toughness: i32,
}

impl Gets for PowerToughness {
    #[atomic_delegate(PowerToughness, Gets)]
    fn kind(self) -> effects::EffectKind {}
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::pt;

    use super::PowerToughness;

    #[test]
    fn test_usage() {
        let boost = PowerToughness {
            power: 2,
            toughness: 1,
        };
        assert_eq!(boost.power, 2);
        assert_eq!(boost.toughness, 1);
        assert_eq!(pt!(+2, +1), boost);
        assert_eq!(cards::PowerToughness::default().power, 0);
    }
}
