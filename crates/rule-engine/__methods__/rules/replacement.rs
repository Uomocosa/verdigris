use crate::rules;
use crate::rules::traits::event::Event;

pub fn replacement<E: Event>(answers: &rules::Rules, event: &E) -> Option<rules::Then> {
    answers.holder_changes(event, rules::Target::event::<E>()).into_iter().find_map(|change| {
        match change {
            rules::Change::Instead { when, then, .. } => {
                answers.eval(&when.bind(event), event, None).then_some(then)
            }
            _ => None,
        }
    })
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::events;
    use crate::rules;
    use crate::testing::*;

    use super::replacement;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, _]) = MockGame::new([P1.battlefield([LABORATORY_MANIAC]), P2]);
        let [maniac] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(LABORATORY_MANIAC)?;
        let player = p1.id;
        game.pass_priority();
        assert!(game.card(maniac).is_in(BATTLEFIELD).of(p1));
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(
                    replacement(&answers, &events::DrawFromEmptyLibrary { player }),
                    Some(rules::Then::Events(vec![rules::EventTemplate::WinGame {
                        who: rules::YOU,
                    }]))
                );
                assert_eq!(replacement(&answers, &events::LoseGame { player }), None);
            })
            .unwrap();
        Ok(())
    }
}
