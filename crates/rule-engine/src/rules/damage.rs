use derive_more::{Deref, From};

use crate::effects;
use crate::rules::traits::dealt::Dealt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, From)]
pub struct Damage(pub u32);

#[rustfmt::skip]
impl Dealt for Damage {
    type Builder = effects::DealsDamage;

    fn build(self) -> Self::Builder { effects::DealsDamage::from(*self) }
}

#[cfg(test)]
mod tests {
    use crate::effects;
    use crate::rules;
    use crate::rules::traits::dealt::Dealt;

    use super::Damage;

    #[test]
    fn test_usage() {
        assert_eq!(rules::damage(3), Damage(3));
        assert_eq!(Damage::from(3u32), Damage(3));
        assert_eq!(rules::damage(3).build(), effects::DealsDamage::from(3u32));
    }
}
