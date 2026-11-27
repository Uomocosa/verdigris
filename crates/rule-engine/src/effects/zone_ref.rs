use atomic_delegate_macros::atomic_delegates;

use crate::rules;
use crate::zones;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZoneRef {
    pub zone: zones::Zone,
    pub of: rules::Players,
}

#[atomic_delegates]
impl From<zones::Zone> for ZoneRef {
    fn from(zone: zones::Zone) -> Self {}
}

#[atomic_delegates]
impl ZoneRef {
    #[must_use]
    pub fn of(self, player: impl Into<rules::Players>) -> Self {}
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::YOU;
    use crate::effects;
    use crate::rules;

    #[test]
    fn test_usage() {
        let hand = effects::ZoneRef {
            zone: HAND,
            of: rules::Players::All,
        };
        assert_eq!(
            hand.of(YOU),
            effects::ZoneRef {
                zone: HAND,
                of: rules::Players::You,
            }
        );
    }
}
