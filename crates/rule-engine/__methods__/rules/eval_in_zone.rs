use bevy::prelude::Entity;

use crate::rules;
use crate::rules::traits::event_args::EventArgs;
use crate::zones;

pub fn eval_in_zone<E: EventArgs>(
    answers: &rules::Rules,
    zone: zones::Zone,
    reference: rules::ArgRef,
    event: &E,
    viewpoint: Option<Entity>,
) -> bool {
    let holders: Vec<Entity> = match answers.resolve(reference, event) {
        Some(rules::ArgValue::Entity(found)) => vec![found],
        Some(rules::ArgValue::Player(player)) => vec![*player],
        Some(rules::ArgValue::Entities(found)) => found,
        Some(rules::ArgValue::Amount(_) | rules::ArgValue::Event(_)) | None => return false,
    };
    viewpoint.is_some_and(|subject| {
        holders.iter().all(|holder| {
            answers
                .entities
                .get(subject)
                .ok()
                .and_then(|entity| entity.get::<zones::InZone>())
                .is_some_and(|in_zone| {
                    answers.zones.get(**in_zone).is_ok_and(|(_, found, owner)| {
                        *found == zone && owner.is_some_and(|owner| ***owner == *holder)
                    })
                })
        })
    })
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::HAND;
    use crate::rules;
    use crate::testing::*;

    use super::eval_in_zone;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, _p2]) =
            MockGame::new([P1.battlefield([GRIZZLY_BEARS]).hand([GIANT_GROWTH]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [growth] = game.player(p1).cards_in(HAND).exactly::<1>(GIANT_GROWTH)?;
        let attack = TestRule {
            attacker: *bears,
            player: p1.id,
        };
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert!(eval_in_zone(
                    &answers,
                    BATTLEFIELD,
                    TestRule::PLAYER.into(),
                    &attack,
                    Some(*bears),
                ));
                assert!(!eval_in_zone(
                    &answers,
                    BATTLEFIELD,
                    TestRule::PLAYER.into(),
                    &attack,
                    Some(*growth),
                ));
            })
            .unwrap();
        Ok(())
    }
}
