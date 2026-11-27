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
        spell_or_ability: spell,
    };
    targeting.eval(&requirement.filter, &context, Some(target)) && targeting.legal(&context)
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::Commands;
    use bevy::prelude::Entity;

    use crate::cards;
    use crate::effects;
    use crate::mana;
    use crate::stack;
    use crate::testing::*;
    use crate::zones;
    use crate::{BATTLEFIELD, HAND, any_target, creature, enchantment};

    use super::is_legal_target;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]),
            P2.battlefield([GRIZZLY_BEARS, WIND_DRAKE]),
        ]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [drake] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(WIND_DRAKE)?;
        game.play([p1.pass_priority()])?;
        let stack_zone = game.stack;
        let source = *bears;
        let ability = game
            .app
            .world_mut()
            .run_system_once(move |mut commands: Commands| {
                commands
                    .spawn((
                        zones::InZone(stack_zone),
                        cards::Controller(p2.id),
                        stack::TriggeredAbilityOnStack {
                            source,
                            intervening_if: None,
                            effect: effects::draw(1),
                        },
                    ))
                    .id()
            })
            .unwrap();
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
                let not_source = effects::target(1, creature() & !effects::Filter::source());
                assert!(!is_legal_target(
                    &targeting,
                    ability,
                    &not_source,
                    Entity::from(bears)
                ));
                assert!(is_legal_target(
                    &targeting,
                    ability,
                    &not_source,
                    Entity::from(drake)
                ));
            })
            .unwrap();
        Ok(())
    }
}
