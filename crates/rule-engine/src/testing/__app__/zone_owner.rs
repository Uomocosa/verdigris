use std::fmt::Display;

use crate::testing;
use crate::zones;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZoneOwner {
    Player(testing::TestPlayer),
    Shared,
}

impl From<testing::TestPlayer> for ZoneOwner {
    fn from(player: testing::TestPlayer) -> Self {
        Self::Player(player)
    }
}

impl Display for ZoneOwner {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Player(player) => write!(formatter, "{}", player.seat),
            Self::Shared => write!(formatter, "the stack"),
        }
    }
}

impl ZoneOwner {
    pub(crate) fn location(self, zone: zones::Zone) -> String {
        match self {
            Self::Player(_) => format!(" in {zone:?}"),
            Self::Shared => " on the stack".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::testing::*;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::new([P1, P2]);
        let owner = ZoneOwner::from(p1);
        assert_eq!(owner, ZoneOwner::Player(p1));
        assert_eq!(format!("{owner}"), "P1");
        assert_eq!(format!("{}{}", owner, owner.location(HAND)), "P1 in Hand");
        assert_eq!(format!("{}", ZoneOwner::Shared), "the stack");
        assert_eq!(game.stack().count(), 0);
    }
}
