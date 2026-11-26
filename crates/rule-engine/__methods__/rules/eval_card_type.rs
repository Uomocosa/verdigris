use bevy::prelude::Entity;

use crate::cards;
use crate::rules;

pub fn eval_card_type(
    answers: &rules::Rules,
    card: cards::CardType,
    viewpoint: Option<Entity>,
) -> bool {
    viewpoint.is_some_and(|subject| {
        answers
            .definitions
            .get(subject)
            .is_ok_and(|definition| definition.type_line.types.contains(&card))
    })
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::HAND;
    use crate::cards;
    use crate::rules;
    use crate::testing::*;

    use super::eval_card_type;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, _p2]) = MockGame::new([P1.hand([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(eval_card_type(
                    &answers,
                    cards::CardType::Creature,
                    Some(*bears)
                ));
                assert!(!eval_card_type(
                    &answers,
                    cards::CardType::Land,
                    Some(*bears)
                ));
                assert!(!eval_card_type(&answers, cards::CardType::Creature, None));
            })
            .unwrap();
        Ok(())
    }
}
