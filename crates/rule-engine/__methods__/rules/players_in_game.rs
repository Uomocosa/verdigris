use crate::players;
use crate::rules;

pub fn players_in_game(
    answers: &rules::Rules,
    set: rules::Players,
    you: Option<players::PlayerId>,
) -> Vec<players::PlayerId> {
    let in_game: Vec<players::PlayerId> = answers
        .entities
        .iter()
        .filter(|entity| {
            entity.get::<players::Player>().is_some()
                && entity.get::<players::LeftTheGame>().is_none()
        })
        .map(|entity| players::PlayerId(entity.id()))
        .collect();
    set.resolve(you, &in_game)
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::rules;
    use crate::testing::*;

    use super::players_in_game;

    #[test]
    fn test_usage() {
        let (mut game, [p1, p2]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let mut all = players_in_game(&answers, rules::Players::All, Some(p1.id));
                all.sort();
                let mut expected = vec![p1.id, p2.id];
                expected.sort();
                assert_eq!(all, expected);
                assert_eq!(
                    players_in_game(&answers, rules::Players::You, Some(p1.id)),
                    vec![p1.id]
                );
                assert_eq!(
                    players_in_game(&answers, rules::Players::Opponents, Some(p1.id)),
                    vec![p2.id]
                );
                assert!(
                    players_in_game(&answers, rules::Players::You, None).is_empty()
                        && players_in_game(&answers, rules::Players::Opponents, None).is_empty()
                );
            })
            .unwrap();
    }
}
