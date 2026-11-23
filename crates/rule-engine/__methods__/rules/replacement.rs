use bevy::prelude::Entity;

use crate::rules;

pub fn replacement(
    answers: &rules::Rules,
    affected: Entity,
    event: rules::EventId,
) -> Option<rules::Outcome> {
    answers.changes(affected, rules::Target::Event(event)).into_iter().find_map(|change| {
        match change {
            rules::Change::Instead(outcome) => Some(outcome),
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
    use crate::rules::traits::event::Event;
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
                    replacement(&answers, player, rules::DRAW_FROM_EMPTY_LIBRARY),
                    Some(rules::Outcome::WinTheGame)
                );
                assert_eq!(replacement(&answers, player, rules::LoseGame::ID), None);
            })
            .unwrap();
        Ok(())
    }
}
