use crate::testing;
use crate::zones;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CardsError {
    #[error("{player} has no {card} in {zone:?}")]
    Missing {
        player: testing::PlayerSeat,
        card: String,
        zone: zones::Zone,
    },
    #[error("{player} has {count} {card} in {zone:?}, expected exactly one")]
    NotOne {
        player: testing::PlayerSeat,
        card: String,
        zone: zones::Zone,
        count: usize,
    },
    #[error("{player} has {count} {card} in {zone:?}, expected exactly {expected}")]
    WrongCount {
        player: testing::PlayerSeat,
        card: String,
        zone: zones::Zone,
        count: usize,
        expected: usize,
    },
    #[error("{player} has {count} {card} in {zone:?}; index {index} is out of range")]
    OutOfRange {
        player: testing::PlayerSeat,
        card: String,
        zone: zones::Zone,
        count: usize,
        index: usize,
    },
}
