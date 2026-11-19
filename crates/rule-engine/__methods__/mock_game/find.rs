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
                    .get::<zones::Owner>()
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
    use crate::testing;
    use crate::zones;

    use super::find;

    #[test]
    fn test_usage() {
        let game = testing::MockGame::builder()
            .hand(testing::Seat::P1, [testing::lightning_bolt()])
            .build();
        let found = find(&game, game.p1, zones::Zone::Hand, "Lightning Bolt");
        assert!(found.is_some());
        assert!(find(&game, game.p1, zones::Zone::Hand, "Murder").is_none());
    }
}
