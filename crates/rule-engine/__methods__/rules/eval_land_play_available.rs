use crate::players;
use crate::rules;
use crate::rules::traits::event_args::EventArgs;

pub fn eval_land_play_available<E: EventArgs>(
    answers: &rules::Rules,
    who: rules::ArgRef,
    event: &E,
) -> bool {
    let Some(rules::ArgValue::Player(player)) = answers.resolve(who, event) else {
        return false;
    };
    let Ok(state) = answers.state.single() else {
        return false;
    };
    answers
        .entities
        .get(*player)
        .ok()
        .and_then(|entity| entity.get::<players::LandsPlayedThisTurn>().copied())
        .is_some_and(|record| {
            answers
                .value::<rules::LandPlays>(*player)
                .allows(u32::from(record.count_for(state.turn.number)))
        })
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::HAND;
    use crate::events;
    use crate::rules;
    use crate::testing::*;

    use super::eval_land_play_available;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([P1.hand([FOREST, FOREST]), P2]);
        let [forest_1, forest_2] = game.player(p1).cards_in(HAND).exactly::<2>(FOREST)?;
        let first = events::Play {
            card: forest_1.into(),
            player: p1.id,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(eval_land_play_available(
                    &answers,
                    events::Play::PLAYER.into(),
                    &first,
                ));
            })
            .unwrap();
        game.play([p1.play_land(forest_1)])?;
        let second = events::Play {
            card: forest_2.into(),
            player: p1.id,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(!eval_land_play_available(
                    &answers,
                    events::Play::PLAYER.into(),
                    &second,
                ));
            })
            .unwrap();
        Ok(())
    }
}
