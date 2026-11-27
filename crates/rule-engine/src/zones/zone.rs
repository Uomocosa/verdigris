use bevy::prelude::Component;

use crate::effects;
use crate::rules;
use crate::zones;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Zone {
    Library,
    Hand,
    Battlefield,
    Graveyard,
    Exile,
    Stack,
}

impl From<zones::SharedZone> for Zone {
    fn from(_: zones::SharedZone) -> Self {
        Self::Stack
    }
}

#[rustfmt::skip]
impl PartialEq<zones::SharedZone> for Zone {
    fn eq(&self, other: &zones::SharedZone) -> bool { *self == Self::from(*other) }
}

#[rustfmt::skip]
impl Zone {
    #[must_use]
    pub fn of(self, player: rules::Players) -> effects::ZoneRef { effects::ZoneRef::from(self).of(player) }
}

#[cfg(test)]
mod tests {
    use crate::effects;
    use crate::rules;
    use crate::testing::*;
    use crate::zones::SharedZone;
    use crate::{BATTLEFIELD, STACK, YOU};

    use super::Zone;

    #[test]
    fn test_usage() {
        assert_eq!(
            BATTLEFIELD.of(YOU),
            effects::ZoneRef {
                zone: Zone::Battlefield,
                of: rules::Players::You,
            }
        );

        let (game, [p1, ..]) = MockGame::new([P1, P2]);
        assert!(game.zone_entity(p1, BATTLEFIELD).is_some());
    }

    #[test]
    fn shared_stack_converts_to_zone_stack() {
        assert_eq!(Zone::from(SharedZone::Stack), Zone::Stack);
        assert_eq!(Zone::Stack, STACK);
    }
}
