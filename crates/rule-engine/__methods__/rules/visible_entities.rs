use bevy::prelude::Entity;

use crate::players;
use crate::rules;
use crate::zones;

pub fn visible_entities(answers: &rules::Rules, viewer: players::PlayerId) -> Vec<Entity> {
    let mut entities: Vec<Entity> = answers
        .entities
        .iter()
        .filter_map(|entity| {
            let in_zone = entity.get::<zones::InZone>()?;
            let (_, zone, owner) = answers.zones.get(**in_zone).ok()?;
            if matches!(
                *zone,
                zones::Zone::Battlefield
                    | zones::Zone::Graveyard
                    | zones::Zone::Exile
                    | zones::Zone::Stack
            ) || *zone == zones::Zone::Hand && owner.is_some_and(|owner| ***owner == *viewer)
            {
                Some(entity.id())
            } else {
                None
            }
        })
        .collect();
    entities.sort();
    entities
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::BATTLEFIELD;
    use crate::HAND;
    use crate::rules;
    use crate::testing::*;

    use super::visible_entities;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS]).hand([FOREST]),
            P2.hand([FOREST]),
        ]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST)?;
        let [stranger] = game.player(p2).cards_in(HAND).exactly::<1>(FOREST)?;
        game.app.update();
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let listed = visible_entities(&answers, p1.id);
                assert!(listed.contains(&bears.into()));
                assert!(listed.contains(&forest.into()));
                assert!(!listed.contains(&stranger.into()));
            })
            .unwrap();
        Ok(())
    }
}
