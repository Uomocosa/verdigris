use std::collections::BTreeSet;

use atomic_delegate_macros::atomic_delegates;

use crate::players;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Priority {
    pub holder: Option<players::PlayerId>,
    pub(crate) passed: BTreeSet<players::PlayerId>,
}

#[atomic_delegates]
impl Priority {
    #[must_use]
    pub const fn held_by(holder: players::PlayerId) -> Self {}
}

#[rustfmt::skip]
impl Priority {
    pub const NO_HOLDER: Self = Self { holder: None, passed: BTreeSet::new() };
}

#[rustfmt::skip]
impl Priority {
    #[must_use]
    pub fn has_passed(&self, player: players::PlayerId) -> bool { self.passed.contains(&player) }
}

impl Priority {
    pub fn give_to(&mut self, player: players::PlayerId) {
        self.holder = Some(player);
        self.passed.clear();
    }

    pub fn pass_to(&mut self, next: players::PlayerId) {
        if let Some(holder) = self.holder {
            self.passed.insert(holder);
        }
        self.holder = Some(next);
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;

    use super::Priority;

    #[test]
    fn test_usage() {
        let (_game, [p1, p2]) = MockGame::new([P1, P2]);
        let mut priority = Priority::held_by(p1.id);
        assert_eq!(priority.holder, Some(p1.id));
        assert!(!priority.has_passed(p1.id));
        priority.pass_to(p2.id);
        assert_eq!(priority.holder, Some(p2.id));
        assert!(priority.has_passed(p1.id));
        assert!(!priority.has_passed(p2.id));
        priority.give_to(p2.id);
        assert_eq!(priority.holder, Some(p2.id));
        assert!(!priority.has_passed(p1.id));
        assert!(!priority.has_passed(p2.id));
    }
}
