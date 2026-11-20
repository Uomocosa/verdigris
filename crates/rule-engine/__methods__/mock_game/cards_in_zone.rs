use bevy::prelude::Entity;

use crate::cards;
use crate::testing;
use crate::zones;

pub fn cards_in_zone(
    game: &testing::MockGame,
    player: Entity,
    zone: zones::Zone,
) -> Vec<(testing::TestCard, cards::CardName)> {
    let world = game.app.world();
    let Some(zone_entity) = world
        .iter_entities()
        .find(|entity| {
            entity.get::<zones::Zone>().is_some_and(|found| *found == zone)
                && entity.get::<cards::Owner>().is_some_and(|owner| **owner == player)
        })
        .map(|entity| entity.id())
    else {
        return Vec::new();
    };
    world
        .iter_entities()
        .filter(|entity| {
            entity.get::<zones::InZone>().is_some_and(|in_zone| **in_zone == zone_entity)
        })
        .filter_map(|entity| {
            let definition = entity.get::<cards::CardDefinition>()?;
            Some((testing::TestCard(entity.id()), definition.name.clone()))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::HAND;
    use crate::testing::*;

    use super::cards_in_zone;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::new([P1.hand([LIGHTNING_BOLT, LIGHTNING_BOLT]), P2]);
        let found = cards_in_zone(&game, *p1, HAND);
        assert_eq!(found.len(), 2);
        let names: Vec<_> = found.iter().map(|(_, name)| name.as_str()).collect();
        assert_eq!(names, ["Lightning Bolt", "Lightning Bolt"]);
        assert_eq!(cards_in_zone(&game, *p1, BATTLEFIELD).len(), 0);
    }
}
