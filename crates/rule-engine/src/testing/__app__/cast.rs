use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::Entity;

use crate::testing;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cast {
    pub(crate) caster: Entity,
    pub(crate) spell: Entity,
    pub(crate) targets: Vec<Entity>,
    pub(crate) should_resolve: bool,
}

#[atomic_delegates]
impl Cast {
    #[must_use]
    pub(crate) fn from_caster_and_spell(caster: Entity, spell: Entity) -> Self {}
}

#[rustfmt::skip]
impl IntoIterator for Cast {
    type Item = Self;
    type IntoIter = std::iter::Once<Self>;

    fn into_iter(self) -> Self::IntoIter { std::iter::once(self) }
}

impl Cast {
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
impl Cast {
    #[must_use]
    pub fn steps(&self) -> Vec<testing::Step> {}
}

#[cfg(test)]
mod tests {
    use bevy::prelude::World;

    use crate::actions;
    use crate::testing;
    use crate::testing::traits::Caster;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let caster = world.spawn_empty().id();
        let spell = world.spawn_empty().id();
        let target = world.spawn_empty().id();

        let cast = caster.cast(spell).targeting(target).resolve();

        assert_eq!(
            cast.steps(),
            vec![
                testing::Step::Act(caster, actions::Action::Cast(spell)),
                testing::Step::Act(caster, actions::Action::ChooseTargets(vec![target])),
                testing::Step::PassPriorityAll,
            ]
        );
    }
}
