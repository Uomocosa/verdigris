use bevy::prelude::Entity;

use crate::cards;
use crate::players;
use crate::rules;
use crate::rules::traits::event_args::EventArgs;

pub fn eval_controlled_by<E: EventArgs>(
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
                entity
                    .get::<cards::Controller>()
                    .is_some_and(|controller| members.contains(&**controller))
            }
        })
    })
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::events;
    use crate::rules;
    use crate::testing::*;

    use super::eval_controlled_by;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [own] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [foe] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let context = events::AbilityContext {
            ability: *own,
            source: *own,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(eval_controlled_by(
                    &answers,
                    rules::Players::You,
                    &context,
                    Some(*own),
                ));
                assert!(!eval_controlled_by(
                    &answers,
                    rules::Players::You,
                    &context,
                    Some(*foe),
                ));
                assert!(eval_controlled_by(
                    &answers,
                    rules::Players::You,
                    &context,
                    Some(*p1.id),
                ));
                assert!(!eval_controlled_by(
                    &answers,
                    rules::Players::You,
                    &context,
                    Some(*p2.id),
                ));
                assert!(!eval_controlled_by(
                    &answers,
                    rules::Players::You,
                    &context,
                    None
                ));
            })
            .unwrap();
        Ok(())
    }
}
