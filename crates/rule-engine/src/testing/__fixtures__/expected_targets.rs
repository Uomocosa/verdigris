use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::Entity;
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectedTargets {
    pub count: u32,
    pub legal: BTreeSet<Entity>,
}

#[atomic_delegates]
impl ExpectedTargets {
    #[must_use]
    pub fn from(&self, targets: impl IntoIterator<Item = Entity>) -> Self {}
}
