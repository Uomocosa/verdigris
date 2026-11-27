use crate::rules;
use crate::rules::traits::event_args::EventArgs;

pub fn eval_any_player<E: EventArgs>(
    answers: &rules::Rules,
    set: rules::Players,
    inner: &rules::Condition,
    event: &E,
) -> bool {
    let you = match answers.resolve(rules::YOU, event) {
        Some(rules::ArgValue::Player(player)) => Some(player),
        Some(_) | None => None,
    };
    answers
        .players_in_game(set, you)
        .iter()
        .any(|player| answers.eval(inner, event, Some(**player)))
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::events;
    use crate::rules;
    use crate::testing::*;

    use super::eval_any_player;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, _]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let context = events::AbilityContext {
            ability: *bears,
            source: *bears,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(eval_any_player(
                    &answers,
                    rules::Players::All,
                    &rules::life().is(rules::Value(20).or_less()),
                    &context,
                ));
                assert!(!eval_any_player(
                    &answers,
                    rules::Players::Opponents,
                    &rules::life().is(rules::Value(19).or_less()),
                    &context,
                ));
            })
            .unwrap();
        Ok(())
    }
}
