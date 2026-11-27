use atomic_delegate_macros::atomic_delegates;
use bevy::ecs::system::SystemParam;
use bevy::prelude::Entity;
use derive_more::Deref;

use crate::effects;
use crate::rules;

#[derive(Clone, Copy, SystemParam, Deref)]
pub struct Targeting<'w, 's>(rules::Rules<'w, 's>);

#[atomic_delegates]
impl Targeting<'_, '_> {
    #[must_use]
    pub fn is_legal_target(
        &self,
        spell: Entity,
        requirement: &effects::Target,
        target: Entity,
    ) -> bool {
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::Entity;

    use crate::effects;
    use crate::mana;
    use crate::stack;
    use crate::testing::*;
    use crate::{BATTLEFIELD, HAND, any_target};

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
                let requirement = effects::target(1, any_target());
                assert!(targeting.is_legal_target(*bolt, &requirement, Entity::from(bears)));
                assert!(targeting.is_legal_target(*bolt, &requirement, *p1));
                assert!(targeting.is_legal_target(*bolt, &requirement, *p2));
            })
            .unwrap();
        Ok(())
    }
}
