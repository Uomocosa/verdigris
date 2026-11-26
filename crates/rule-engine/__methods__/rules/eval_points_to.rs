use bevy::prelude::Entity;

use crate::rules;
use crate::rules::traits::event_args::EventArgs;

pub fn eval_points_to<E: EventArgs>(
    answers: &rules::Rules,
    component: &rules::ComponentRef,
    reference: rules::ArgRef,
    event: &E,
    viewpoint: Option<Entity>,
) -> bool {
    let wanted = match answers.resolve(reference, event) {
        Some(rules::ArgValue::Entity(found)) => Some(found),
        Some(rules::ArgValue::Player(player)) => Some(*player),
        Some(rules::ArgValue::Entities(_) | rules::ArgValue::Amount(_)) | None => None,
    };
    viewpoint.is_some_and(|subject| {
        wanted.is_some_and(|wanted| {
            answers
                .entities
                .get(subject)
                .ok()
                .and_then(|entity| component.target.and_then(|read| read(&entity)))
                .is_some_and(|found| found == wanted)
        })
    })
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::cards;
    use crate::rules;
    use crate::testing::*;

    use super::eval_points_to;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, _p2]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let attack = TestRule {
            attacker: *bears,
            player: p1.id,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(eval_points_to(
                    &answers,
                    &rules::ComponentRef::relation::<cards::Owner>(),
                    TestRule::PLAYER.into(),
                    &attack,
                    Some(*bears),
                ));
            })
            .unwrap();
        Ok(())
    }
}
