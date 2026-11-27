use bevy::prelude::Entity;

use crate::cards;
use crate::rules;

pub fn eval_characteristic(
    answers: &rules::Rules,
    characteristic: rules::Characteristic,
    comparison: rules::Comparison,
    viewpoint: Option<Entity>,
) -> bool {
    viewpoint.is_some_and(|subject| {
        answers
            .entities
            .get(subject)
            .ok()
            .and_then(|entity| entity.get::<cards::EffectivePT>())
            .is_some_and(|effective| {
                comparison.holds(match characteristic {
                    rules::Characteristic::Power => effective.power,
                    rules::Characteristic::Toughness => effective.toughness,
                })
            })
    })
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::rules;
    use crate::testing::*;

    use super::eval_characteristic;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, _p2]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.app.update();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(eval_characteristic(
                    &answers,
                    rules::Characteristic::Power,
                    rules::Comparison::OrMore(2),
                    Some(*bears),
                ));
                assert!(!eval_characteristic(
                    &answers,
                    rules::Characteristic::Power,
                    rules::Comparison::OrMore(3),
                    Some(*bears),
                ));
            })
            .unwrap();
        Ok(())
    }
}
