use bevy::prelude::Component;

use crate::effects;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Zone {
    Library,
    Hand,
    Battlefield,
    Graveyard,
    Exile,
    Stack,
}

#[rustfmt::skip]
impl Zone {
    #[must_use]
    pub fn of(self, player: effects::PlayerRef) -> effects::ZoneRef { effects::ZoneRef::from(self).of(player) }
}

#[cfg(test)]
mod tests {
    use crate::effects;
    use crate::{BATTLEFIELD, YOU};

    use super::Zone;

    #[test]
    fn test_usage() {
        assert_eq!(
            BATTLEFIELD.of(YOU),
            effects::ZoneRef {
                zone: Zone::Battlefield,
                of: effects::PlayerRef::You,
            }
        );
    }
}
