use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;

use crate::testing;

#[derive(Debug, Clone, Copy, Deref)]
pub struct PassBudget(pub(crate) u32);

impl PassBudget {
    #[must_use]
    pub const fn of(passes: u32) -> Self {
        Self(passes)
    }
}

#[atomic_delegates]
impl PassBudget {
    pub(crate) const fn record_pass(&mut self) -> Result<(), testing::PlayError> {}
}

#[cfg(test)]
mod tests {
    use crate::testing;

    use super::PassBudget;

    #[test]
    fn test_usage() {
        let mut budget = PassBudget::of(2);

        assert!(budget.record_pass().is_ok());
        assert_eq!(budget.record_pass(), Err(testing::PlayError::Stalled));
    }
}
