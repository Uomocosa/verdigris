use atomic_delegate_macros::atomic_delegates;
use derive_more::{Deref, From};

use crate::abilities;
use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, From)]
pub struct DealsDamage(pub bool);

#[atomic_delegates]
impl DealsDamage {
    #[must_use]
    pub fn to(self, recipient: impl Into<rules::An>) -> abilities::TriggerEvent {}
}

#[cfg(test)]
mod tests {
    use super::DealsDamage;

    #[test]
    fn test_usage() {
        let deals = DealsDamage(true);
        assert!(*deals);
    }
}
