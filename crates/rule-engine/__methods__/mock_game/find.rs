use bevy::prelude::Entity;

use crate::cards;
use crate::testing;
use crate::zones;

pub fn find(
    game: &testing::MockGame,
    player: Entity,
    zone: zones::Zone,
    name: &str,
) -> Option<Entity> {
    let world = game.app.world();
    let zone_entity = world
        .iter_entities()
        .find(|entity| {
            entity
                .get::<zones::Zone>()
                .is_some_and(|found| *found == zone)
                && entity
                    .get::<cards::Owner>()
                    .is_some_and(|owner| **owner == player)
        })
        .map(|entity| entity.id())?;
    world
        .iter_entities()
        .find(|entity| {
            entity
                .get::<zones::InZone>()
                .is_some_and(|in_zone| **in_zone == zone_entity)
                && entity
                    .get::<cards::CardDefinition>()
                    .is_some_and(|definition| definition.name.as_str() == name)
        })
        .map(|entity| entity.id())
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::testing::*;

    use super::find;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::new([P1.hand([lightning_bolt()]), P2]);
        let found = find(&game, p1, HAND, "Lightning Bolt");
        assert!(found.is_some());
        assert!(find(&game, p1, HAND, "Murder").is_none());
    }
}
