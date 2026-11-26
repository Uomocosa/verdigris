use bevy::prelude::Entity;

use crate::cards;
use crate::keywords;
use crate::rules;

pub fn eval_has_keyword(
    answers: &rules::Rules,
    id: keywords::KeywordId,
    viewpoint: Option<Entity>,
) -> bool {
    viewpoint.is_some_and(|subject| {
        answers.entities.get(subject).is_ok_and(|entity| {
            entity.contains::<cards::OnBattlefield>()
                && answers.characteristics(subject).keywords.iter().any(|keyword| keyword.id == id)
        })
    })
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::keywords;
    use crate::rules;
    use crate::testing::*;

    use super::eval_has_keyword;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, _p2]) =
            MockGame::new([P1.battlefield([WIND_DRAKE, GRIZZLY_BEARS]), P2]);
        let [drake] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(WIND_DRAKE)?;
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.app.update();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(eval_has_keyword(
                    &answers,
                    keywords::flying::ID,
                    Some(*drake)
                ));
                assert!(!eval_has_keyword(
                    &answers,
                    keywords::flying::ID,
                    Some(*bears)
                ));
                assert!(!eval_has_keyword(&answers, keywords::flying::ID, None));
            })
            .unwrap();
        Ok(())
    }
}
