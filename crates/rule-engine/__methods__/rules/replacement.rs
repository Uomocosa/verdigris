use crate::rules;
use crate::rules::traits::event::Event;

pub fn replacement<E: Event>(
    answers: &rules::Rules,
    event: &E,
) -> Option<Vec<rules::EventTemplate>> {
    let affected = event.player();
    let subject = event.subject();
    let other = event.other();
    answers.changes(affected, rules::Target::Event(E::ID)).into_iter().find_map(|change| {
        match change {
            rules::Change::Instead { when, then } => {
                answers.eval(&when, subject, other, Some(affected)).then_some(then)
            }
            _ => None,
        }
    })
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::Entity;

    use crate::BATTLEFIELD;
    use crate::rules;
    use crate::testing::*;

    use super::replacement;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, _]) = MockGame::new([P1.battlefield([LABORATORY_MANIAC]), P2]);
        let [maniac] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(LABORATORY_MANIAC)?;
        let player = Entity::from(p1);
        game.pass_priority();
        assert!(game.player(p1).cards_in(BATTLEFIELD).has(maniac));
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(
                    replacement(&answers, &rules::DrawFromEmptyLibrary(player)),
                    Some(vec![rules::EventTemplate::WinGame {
                        who: rules::Role::Player,
                    }])
                );
                assert_eq!(replacement(&answers, &rules::LoseGame(player)), None);
            })
            .unwrap();
        Ok(())
    }
}
