use crate::testing;
use crate::zones;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CardsError {
    #[error("{owner} has no {card}{location}", location = owner.location(*zone))]
    Missing {
        owner: testing::ZoneOwner,
        card: String,
        zone: zones::Zone,
    },
    #[error("{owner} has {count} {card}{location}, expected exactly {expected}", location = owner.location(*zone))]
    WrongCount {
        owner: testing::ZoneOwner,
        card: String,
        zone: zones::Zone,
        count: usize,
        expected: usize,
    },
    #[error("{owner} has {count} {card}{location}; index {index} is out of range", location = owner.location(*zone))]
    OutOfRange {
        owner: testing::ZoneOwner,
        card: String,
        zone: zones::Zone,
        count: usize,
        index: usize,
    },
}
