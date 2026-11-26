use bevy::prelude::Entity;

use crate::rules;

pub fn eval_has(
    answers: &rules::Rules,
    component: &rules::ComponentRef,
    viewpoint: Option<Entity>,
) -> bool {
    viewpoint.is_some_and(|subject| {
        answers.entities.get(subject).is_ok_and(|entity| entity.contains_type_id(component.type_id))
    })
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::cards;
    use crate::rules;
    use crate::testing::*;

    use super::eval_has;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [_p1, p2]) = MockGame::new([P1, P2.battlefield_tapped([GRIZZLY_BEARS])]);
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(eval_has(
                    &answers,
                    &rules::ComponentRef::of::<cards::Tapped>(),
                    Some(*bears)
                ));
                assert!(!eval_has(
                    &answers,
                    &rules::ComponentRef::of::<cards::Tapped>(),
                    None
                ));
            })
            .unwrap();
        Ok(())
    }
}
