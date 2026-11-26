use crate::rules;
use crate::rules::traits::event::Event;

pub fn hooks<E: Event>(answers: &rules::Rules, event: &E) -> Vec<rules::Then> {
    answers
        .holder_changes(event, rules::Target::event::<E>())
        .into_iter()
        .filter_map(|change| match change {
            rules::Change::Hook { when, then, .. } => {
                let bound = when.bind(event);
                answers.eval(&bound, event, None).then_some(then)
            }
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::events;
    use crate::rules;
    use crate::testing::*;

    use super::hooks;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, _]) = MockGame::new([P1.battlefield([LABORATORY_MANIAC]), P2]);
        let [maniac] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(LABORATORY_MANIAC)?;
        let player = p1.id;
        game.pass_priority();
        assert!(game.player(p1).cards_in(BATTLEFIELD).has(maniac));
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(
                    hooks(&answers, &events::DrawFromEmptyLibrary { player }),
                    Vec::new()
                );
                assert_eq!(hooks(&answers, &events::LoseGame { player }), Vec::new());
            })
            .unwrap();
        Ok(())
    }
}
