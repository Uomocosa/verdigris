use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::Entity;

use crate::testing;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestPlayerActions {
    pub(crate) caster: Entity,
    pub(crate) spell: Entity,
    pub(crate) targets: Vec<Entity>,
    pub(crate) should_resolve: bool,
}

#[atomic_delegates]
impl TestPlayerActions {
    #[must_use]
    pub(crate) const fn from_caster_and_spell(caster: Entity, spell: Entity) -> Self {}
}

#[rustfmt::skip]
impl IntoIterator for TestPlayerActions {
    type Item = Self;
    type IntoIter = std::iter::Once<Self>;

    fn into_iter(self) -> Self::IntoIter { std::iter::once(self) }
}

impl TestPlayerActions {
    #[must_use]
    pub fn targeting(mut self, targets: impl Into<testing::Targets>) -> Self {
        self.targets = targets.into().into_entities();
        self
    }

    #[must_use]
    pub const fn resolve(mut self) -> Self {
        self.should_resolve = true;
        self
    }
}

#[atomic_delegates]
impl TestPlayerActions {
    #[must_use]
    pub fn steps(&self) -> Vec<testing::PlayStep> {}
}

#[cfg(test)]
mod tests {
    use bevy::prelude::World;

    use crate::actions;
    use crate::testing;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let caster = world.spawn_empty().id();
        let spell = world.spawn_empty().id();
        let target = world.spawn_empty().id();

        let cast = testing::TestPlayer(caster)
            .cast(spell)
            .targeting(target)
            .resolve();

        assert_eq!(
            cast.steps(),
            vec![
                testing::PlayStep::Act(caster, actions::Action::Cast(spell)),
                testing::PlayStep::Act(caster, actions::Action::ChooseTargets(vec![target])),
                testing::PlayStep::PassPriorityAll,
            ]
        );
    }
}
