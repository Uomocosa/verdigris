use atomic_delegate_macros::atomic_delegates;

use crate::effects;
use crate::zones;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZoneRef {
    pub zone: zones::Zone,
    pub of: effects::PlayerRef,
}

#[atomic_delegates]
impl ZoneRef {
    #[must_use]
    pub fn of(self, player: effects::PlayerRef) -> Self {}
}

#[cfg(test)]
mod tests {
    use crate::effects;
    use crate::zones;

    #[test]
    fn test_usage() {
        let hand = effects::ZoneRef {
            zone: zones::Zone::Hand,
            of: effects::PlayerRef::Any,
        };
        assert_eq!(
            hand.of(effects::PlayerRef::You),
            effects::ZoneRef {
                zone: zones::Zone::Hand,
                of: effects::PlayerRef::You,
            }
        );
    }
}
