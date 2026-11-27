use bevy::prelude::Entity;

use crate::cards;
use crate::rules;

pub fn eval_supertype(
    answers: &rules::Rules,
    supertype: cards::Supertype,
    viewpoint: Option<Entity>,
) -> bool {
    viewpoint.is_some_and(|subject| {
        answers
            .definitions
            .get(subject)
            .is_ok_and(|definition| definition.type_line.supertypes.contains(&supertype))
    })
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::HAND;
    use crate::cards;
    use crate::rules;
    use crate::testing::*;

    use super::eval_supertype;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, _p2]) = MockGame::new([P1.hand([FOREST, GRIZZLY_BEARS]), P2]);
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST)?;
        let [bears] = game.player(p1).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(eval_supertype(
                    &answers,
                    cards::Supertype::Basic,
                    Some(*forest)
                ));
                assert!(!eval_supertype(
                    &answers,
                    cards::Supertype::Basic,
                    Some(*bears)
                ));
                assert!(!eval_supertype(&answers, cards::Supertype::Basic, None));
            })
            .unwrap();
        Ok(())
    }
}
