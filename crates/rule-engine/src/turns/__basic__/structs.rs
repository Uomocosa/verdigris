use crate::players;
use crate::turns;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Turn {
    pub number: u32,
    pub active: players::PlayerId,
    pub step: turns::Step,
}
