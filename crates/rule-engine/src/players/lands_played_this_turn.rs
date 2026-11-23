use bevy::prelude::Component;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct LandsPlayedThisTurn {
    pub count: u8,
    pub turn_number: u32,
}

impl LandsPlayedThisTurn {
    pub const NONE: Self = Self {
        count: 0,
        turn_number: 0,
    };
}

impl LandsPlayedThisTurn {
    #[must_use]
    pub const fn count_for(&self, turn_number: u32) -> u8 {
        if self.turn_number == turn_number {
            self.count
        } else {
            0
        }
    }
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
        assert_eq!(played.count_for(1), 0);
        played.record_play(1);
        assert_eq!(played.count_for(1), 1);
        assert_eq!(played.count_for(2), 0);
    }
}
