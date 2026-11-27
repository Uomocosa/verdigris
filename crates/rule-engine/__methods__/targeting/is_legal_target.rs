use bevy::prelude::Entity;

use crate::effects;
use crate::events;
use crate::stack;

pub fn is_legal_target(
    targeting: &stack::Targeting,
    spell: Entity,
    requirement: &effects::Target,
    target: Entity,
) -> bool {
    if target == spell {
        return false;
    }
    let context = events::BecomeTarget {
        target,
        spell,
        ability: spell,
    };
    targeting.answers.eval(&requirement.filter, &context, Some(target))
        && targeting.answers.legal(&context)
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::Entity;

    use crate::effects;
    use crate::mana;
    use crate::stack;
    use crate::testing::*;
    use crate::{BATTLEFIELD, HAND, any_target, enchantment};

    use super::is_legal_target;

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
                assert!(is_legal_target(
                    &targeting,
                    *bolt,
                    &requirement,
                    Entity::from(bears)
                ));
                assert!(!is_legal_target(
                    &targeting,
                    Entity::from(bears),
                    &effects::target(1, any_target()),
                    Entity::from(bears)
                ));
                let enchantment = effects::target(1, enchantment());
                assert!(!is_legal_target(
                    &targeting,
                    *bolt,
                    &enchantment,
                    Entity::from(bears)
                ));
            })
            .unwrap();
        Ok(())
    }
}
