use bevy::prelude::Entity;

use crate::cards;
use crate::testing;
use crate::zones;

pub fn entities_in_zone_matching_card(
    game: &testing::MockGame,
    player: Entity,
    zone: zones::Zone,
    name: &str,
) -> Vec<Entity> {
    let world = game.app.world();
    let Some(zone_entity) = world
        .iter_entities()
        .find(|entity| {
            entity
                .get::<zones::Zone>()
                .is_some_and(|found| *found == zone)
                && entity
                    .get::<cards::Owner>()
                    .is_some_and(|owner| **owner == player)
        })
        .map(|entity| entity.id())
    else {
        return Vec::new();
    };
    world
        .iter_entities()
        .filter(|entity| {
            entity
                .get::<zones::InZone>()
                .is_some_and(|in_zone| **in_zone == zone_entity)
                && entity
                    .get::<cards::CardDefinition>()
                    .is_some_and(|definition| definition.name.as_str() == name)
        })
        .map(|entity| entity.id())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::testing::*;

    use super::entities_in_zone_matching_card;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::new([P1.hand([LIGHTNING_BOLT, LIGHTNING_BOLT]), P2]);
        let found = entities_in_zone_matching_card(&game, *p1, HAND, "Lightning Bolt");
        assert_eq!(found.len(), 2);
        assert_eq!(
            entities_in_zone_matching_card(&game, *p1, HAND, "Murder"),
            Vec::<bevy::prelude::Entity>::new()
        );
    }
}
