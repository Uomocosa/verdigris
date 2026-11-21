use bevy::prelude::Entity;

use crate::zones;

pub fn find(
    lookup: &zones::ZoneLookup,
    zone: impl Into<zones::Zone>,
    owner: Option<Entity>,
) -> Option<Entity> {
    let zone = zone.into();
    lookup.iter().find_map(|(entity, found_zone, found_owner)| {
        if *found_zone != zone {
            return None;
        }
        if let Some(owner) = owner
            && found_owner.is_none_or(|found_owner| **found_owner != owner)
        {
            return None;
        }
        Some(entity)
    })
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::Entity;

    use crate::testing::*;
    use crate::{HAND, STACK};

    use super::find;
    use crate::zones;

    #[test]
    fn test_usage() {
        let (mut game, [p1, ..]) = MockGame::new([P1, P2]);
        let (stack, hand, owner) = (
            game.stack,
            game.zone_entity(p1, HAND).unwrap(),
            Entity::from(p1),
        );
        game.app
            .world_mut()
            .run_system_once(move |lookup: zones::ZoneLookup| {
                assert_eq!(find(&lookup, zones::Zone::Stack, None), Some(stack));
                assert_eq!(find(&lookup, zones::Zone::Hand, Some(owner)), Some(hand));
                assert_eq!(find(&lookup, zones::Zone::Stack, Some(stack)), None);
                assert_eq!(
                    find(&lookup, STACK, None),
                    find(&lookup, zones::Zone::Stack, None)
                );
            })
            .unwrap();
    }
}
