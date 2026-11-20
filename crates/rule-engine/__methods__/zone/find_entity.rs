use bevy::prelude::{Entity, Query};

use crate::cards;
use crate::zones;

pub fn find_entity(
    zone: zones::Zone,
    owner: Option<Entity>,
    zone_entities: &Query<(Entity, &zones::Zone, Option<&cards::Owner>)>,
) -> Option<Entity> {
    for (entity, found_zone, found_owner) in zone_entities.iter() {
        if *found_zone != zone {
            continue;
        }
        if let Some(owner) = owner
            && found_owner.is_none_or(|found_owner| **found_owner != owner)
        {
            continue;
        }
        return Some(entity);
    }
    None
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::HAND;
    use crate::cards;
    use crate::testing::*;
    use crate::zones;

    use super::find_entity;

    #[test]
    fn test_usage() {
        let (mut game, [p1, ..]) = MockGame::new([P1, P2]);
        let (stack, hand, owner) = (
            game.stack,
            game.zone_entity(p1, HAND).unwrap(),
            Entity::from(p1),
        );
        let (stack_found, hand_found, owned_stack) = game.with_query::<(
            Entity,
            &zones::Zone,
            Option<&cards::Owner>,
        ), (
            Option<Entity>,
            Option<Entity>,
            Option<Entity>,
        )>(|zone_entities| {
            (
                find_entity(zones::Zone::Stack, None, &zone_entities),
                find_entity(zones::Zone::Hand, Some(owner), &zone_entities),
                find_entity(zones::Zone::Stack, Some(stack), &zone_entities),
            )
        });
        assert_eq!(stack_found, Some(stack));
        assert_eq!(hand_found, Some(hand));
        assert_eq!(owned_stack, None);
    }
}
