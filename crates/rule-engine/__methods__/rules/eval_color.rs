use bevy::prelude::Entity;

use crate::cards;
use crate::rules;

pub fn eval_color(answers: &rules::Rules, color: cards::Color, viewpoint: Option<Entity>) -> bool {
    viewpoint.is_some_and(|subject| {
        answers
            .definitions
            .get(subject)
            .is_ok_and(|definition| definition.colors().contains(&color))
    })
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::HAND;
    use crate::cards;
    use crate::rules;
    use crate::testing::*;

    use super::eval_color;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, _p2]) = MockGame::new([P1.hand([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(eval_color(&answers, cards::Color::Green, Some(*bears)));
                assert!(!eval_color(&answers, cards::Color::White, Some(*bears)));
                assert!(!eval_color(&answers, cards::Color::Green, None));
            })
            .unwrap();
        Ok(())
    }
}
