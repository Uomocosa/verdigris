use bevy::prelude::Entity;

use crate::cards;
use crate::damage;
use crate::rules;

pub fn remaining_toughness(answers: &rules::Rules, subject: Entity) -> u32 {
    answers
        .entities
        .get(subject)
        .ok()
        .and_then(|entity| entity.get::<cards::EffectivePT>())
        .map_or(0, |effective| {
            u32::try_from(effective.toughness).unwrap_or_default()
        })
        .saturating_sub(
            answers
                .entities
                .get(subject)
                .ok()
                .and_then(|entity| entity.get::<damage::Damage>())
                .map_or(0, |marked| **marked),
        )
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::rules;
    use crate::testing::*;

    use super::remaining_toughness;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, _]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.app.update();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                assert_eq!(remaining_toughness(&answers, *bears), 2);
            })
            .unwrap();
        Ok(())
    }
}
