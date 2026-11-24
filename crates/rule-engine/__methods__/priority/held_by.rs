use std::collections::BTreeSet;

use crate::players;
use crate::turns;

pub const fn held_by(holder: players::PlayerId) -> turns::Priority {
    turns::Priority {
        holder: Some(holder),
        passed: BTreeSet::new(),
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;

    use super::held_by;

    #[test]
    fn test_usage() {
        let (_game, [p1, ..]) = MockGame::new([P1, P2]);
        let priority = held_by(p1.id);
        assert_eq!(priority.holder, Some(p1.id));
        assert!(priority.passed.is_empty());
    }
}
