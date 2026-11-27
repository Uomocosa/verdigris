use derive_more::{Deref, From};

use crate::abilities;
use crate::rules::traits::dealt::Dealt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, From)]
pub struct DamageFilter(pub bool);

pub const DAMAGE: DamageFilter = DamageFilter(false);
pub const COMBAT_DAMAGE: DamageFilter = DamageFilter(true);

#[rustfmt::skip]
impl Dealt for DamageFilter {
    type Builder = abilities::DealsDamage;

    fn build(self) -> Self::Builder { abilities::DealsDamage::from(*self) }
}

#[cfg(test)]
mod tests {
    use crate::abilities;
    use crate::rules;
    use crate::rules::traits::dealt::Dealt;

    use super::COMBAT_DAMAGE;
    use super::DAMAGE;
    use super::DamageFilter;

    #[test]
    fn test_usage() {
        assert_eq!(COMBAT_DAMAGE, DamageFilter(true));
        assert_eq!(DAMAGE, DamageFilter(false));
        assert_eq!(COMBAT_DAMAGE.build(), abilities::DealsDamage::from(true));
        assert_eq!(rules::DAMAGE.build(), abilities::DealsDamage::from(false));
    }
}
