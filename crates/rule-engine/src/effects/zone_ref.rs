use atomic_delegate_macros::atomic_delegates;

use crate::effects;
use crate::zones;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZoneRef {
    pub zone: zones::Zone,
    pub of: effects::PlayerRef,
}

#[atomic_delegates]
impl From<zones::Zone> for ZoneRef {
    fn from(zone: zones::Zone) -> Self {}
}

#[atomic_delegates]
impl ZoneRef {
    #[must_use]
    pub fn of(self, player: effects::PlayerRef) -> Self {}
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::effects;

    #[test]
    fn test_usage() {
        let hand = effects::ZoneRef {
            zone: HAND,
            of: effects::PlayerRef::Any,
        };
        assert_eq!(
            hand.of(effects::PlayerRef::You),
            effects::ZoneRef {
                zone: HAND,
                of: effects::PlayerRef::You,
            }
        );
    }
}
