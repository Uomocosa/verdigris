use crate::zones;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CardsError {
    #[error("P{player} has no {card} in {zone:?}")]
    Missing {
        player: usize,
        card: &'static str,
        zone: zones::Zone,
    },
    #[error("P{player} has {count} {card} in {zone:?}, expected exactly one")]
    NotOne {
        player: usize,
        card: &'static str,
        zone: zones::Zone,
        count: usize,
    },
    #[error("P{player} has {count} {card} in {zone:?}; index {index} is out of range")]
    OutOfRange {
        player: usize,
        card: &'static str,
        zone: zones::Zone,
        count: usize,
        index: usize,
    },
}
