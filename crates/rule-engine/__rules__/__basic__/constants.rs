use crate::zones;

/// 514.1 Maximum hand size (normally seven).
pub const MAX_HAND_SIZE: usize = 7;

pub const HAND: zones::Zone = zones::Zone::Hand;
pub const BATTLEFIELD: zones::Zone = zones::Zone::Battlefield;
pub const GRAVEYARD: zones::Zone = zones::Zone::Graveyard;
pub const LIBRARY: zones::Zone = zones::Zone::Library;
pub const STACK: zones::Zone = zones::Zone::Stack;
