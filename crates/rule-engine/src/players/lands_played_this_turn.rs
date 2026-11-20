use bevy::prelude::Component;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct LandsPlayedThisTurn {
    pub count: u8,
    pub turn_number: u32,
}

const LANDS_PER_TURN: u8 = 1;

#[rustfmt::skip]
impl LandsPlayedThisTurn {
    pub const NONE: Self = Self { count: 0, turn_number: 0 };

    #[must_use]
    pub const fn has_played_all_this_turn(&self, turn_number: u32) -> bool { self.turn_number == turn_number && self.count >= LANDS_PER_TURN }
}

impl LandsPlayedThisTurn {
    pub const fn record_play(&mut self, turn_number: u32) {
        self.count = if self.turn_number == turn_number {
            self.count.saturating_add(1)
        } else {
            1
        };
        self.turn_number = turn_number;
    }
}

#[cfg(test)]
mod tests {
    use super::LandsPlayedThisTurn;

    #[test]
    fn test_usage() {
        let mut played = LandsPlayedThisTurn::NONE;
        assert!(!played.has_played_all_this_turn(1));
        played.record_play(1);
        assert!(played.has_played_all_this_turn(1));
        assert!(!played.has_played_all_this_turn(2));
    }
}
