use bevy::prelude::Entity;

use crate::rules;
use crate::rules::traits::event::Event;

pub fn replacement<E: Event>(answers: &rules::Rules, event: &E) -> Option<rules::Then> {
    let mut visited: Vec<(rules::ArgRef, Entity)> = Vec::new();
    for id in E::args() {
        for reference in [
            rules::ArgRef::Arg(*id),
            rules::ArgRef::ControllerOf(*id),
            rules::ArgRef::OwnerOf(*id),
        ] {
            let Some(entity) = answers.resolve(reference, event).and_then(|value| match value {
                rules::ArgValue::Entity(entity) => Some(entity),
                rules::ArgValue::Player(player) => Some(*player),
                rules::ArgValue::Entities(_) | rules::ArgValue::Amount(_) => None,
            }) else {
                continue;
            };
            if visited.contains(&(reference, entity)) {
                continue;
            }
            visited.push((reference, entity));
            for change in answers.changes(entity, rules::Target::event::<E>()) {
                if let rules::Change::Instead { when, then } = change
                    && answers.eval(&when.bind(event), event, None)
                {
                    return Some(then);
                }
            }
        }
    }
    None
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
        assert!(game.player(p1).cards_in(BATTLEFIELD).has(maniac));
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(
                    replacement(&answers, &events::DrawFromEmptyLibrary { player }),
                    Some(rules::Then::Events(vec![rules::EventTemplate::WinGame {
                        who: events::DrawFromEmptyLibrary::PLAYER.into(),
                    }]))
                );
                assert_eq!(replacement(&answers, &events::LoseGame { player }), None);
            })
            .unwrap();
        Ok(())
    }
}
