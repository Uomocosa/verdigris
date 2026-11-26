use atomic_delegate_macros::atomic_delegates;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Entity, Query};
use std::collections::BTreeSet;

use crate::effects;
use crate::players;
use crate::rules;

#[derive(SystemParam, Clone, Copy)]
pub struct Targeting<'w, 's> {
    pub(crate) answers: rules::Rules<'w, 's>,
    pub(crate) players: Query<'w, 's, Entity, players::StillInTheGame>,
}

#[atomic_delegates]
impl Targeting<'_, '_> {
    #[must_use]
    pub fn is_legal_target(
        &self,
        spell: Entity,
        requirement: effects::Target,
        controller: players::PlayerId,
        target: Entity,
    ) -> bool {
    }

    #[must_use]
    pub fn legal_targets(
        &self,
        spell: Entity,
        requirement: effects::Target,
        controller: players::PlayerId,
    ) -> BTreeSet<Entity> {
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::Entity;
    use std::collections::BTreeSet;

    use crate::effects;
    use crate::mana;
    use crate::stack;
    use crate::testing::*;
    use crate::{ANY_TARGET, BATTLEFIELD, HAND};

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([p1.pass_priority()])?;
        game.app
            .world_mut()
            .run_system_once(move |targeting: stack::Targeting| {
                let requirement = effects::target(1, ANY_TARGET);
                assert_eq!(
                    targeting.legal_targets(*bolt, requirement, p1.id),
                    BTreeSet::from([*p1, *p2, Entity::from(bears)])
                );
                assert!(targeting.is_legal_target(*bolt, requirement, p1.id, Entity::from(bears)));
            })
            .unwrap();
        Ok(())
    }
}
