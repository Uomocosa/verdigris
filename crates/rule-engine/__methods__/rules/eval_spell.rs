use bevy::prelude::Entity;

use crate::STACK;
use crate::rules;
use crate::stack;
use crate::zones;

pub fn eval_spell(answers: &rules::Rules, viewpoint: Option<Entity>) -> bool {
    viewpoint.is_some_and(|subject| {
        answers.definitions.get(subject).is_ok()
            && answers.entities.get(subject).is_ok_and(|entity| {
                entity
                    .get::<zones::InZone>()
                    .is_some_and(|in_zone| answers.zones.contains(STACK, **in_zone))
                    && entity.get::<stack::TriggeredAbilityOnStack>().is_none()
            })
    })
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::HAND;
    use crate::mana;
    use crate::rules;
    use crate::testing::*;

    use super::eval_spell;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        game.play([p1.cast(bolt).targeting(p2)])?;
        let [stacked] = game.stack().exactly::<1>(LIGHTNING_BOLT)?;
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(eval_spell(&answers, Some(*stacked)));
                assert!(!eval_spell(&answers, Some(*bears)));
                assert!(!eval_spell(&answers, None));
            })
            .unwrap();
        Ok(())
    }
}
