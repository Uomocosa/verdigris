use bevy::prelude::Entity;

use crate::cards;
use crate::players;
use crate::rules;
use crate::rules::traits::event_args::EventArgs;

pub fn eval_owned_by<E: EventArgs>(
    answers: &rules::Rules,
    set: rules::Players,
    event: &E,
    viewpoint: Option<Entity>,
) -> bool {
    let you = answers.resolve(rules::YOU, event).and_then(|value| match value {
        rules::ArgValue::Player(player) => Some(player),
        _ => None,
    });
    let members = answers.players_in_game(set, you);
    viewpoint.is_some_and(|subject| {
        answers.entities.get(subject).is_ok_and(|entity| {
            if entity.get::<players::Player>().is_some() {
                members.contains(&players::PlayerId(subject))
            } else {
                entity.get::<cards::Owner>().is_some_and(|owner| members.contains(&**owner))
            }
        })
    })
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::HAND;
    use crate::events;
    use crate::rules;
    use crate::testing::*;

    use super::eval_owned_by;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, _]) =
            MockGame::new([P1.hand([FOREST]).battlefield([GRIZZLY_BEARS]), P2]);
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST)?;
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let context = events::AbilityContext {
            spell_or_ability: *bears,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(eval_owned_by(
                    &answers,
                    rules::Players::You,
                    &context,
                    Some(*forest),
                ));
                assert!(!eval_owned_by(
                    &answers,
                    rules::Players::Opponents,
                    &context,
                    Some(*forest),
                ));
                assert!(!eval_owned_by(
                    &answers,
                    rules::Players::You,
                    &context,
                    None
                ));
            })
            .unwrap();
        Ok(())
    }

    #[test]
    fn owned_by_opponents_is_true_for_a_foes_card() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([GRIZZLY_BEARS]), P2.battlefield([GRIZZLY_BEARS])]);
        let [own] = game.player(p1).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;
        let [foe] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let context = events::AbilityContext {
            spell_or_ability: *foe,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(eval_owned_by(
                    &answers,
                    rules::Players::Opponents,
                    &context,
                    Some(*own),
                ));
                assert!(!eval_owned_by(
                    &answers,
                    rules::Players::Opponents,
                    &context,
                    Some(*foe),
                ));
            })
            .unwrap();
        Ok(())
    }
}
