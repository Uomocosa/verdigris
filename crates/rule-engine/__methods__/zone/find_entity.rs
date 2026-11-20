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
    use bevy::ecs::system::SystemState;
    use bevy::prelude::{Entity, Query, World};

    use crate::cards;
    use crate::zones;

    use super::find_entity;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let stack_entity = world.spawn(zones::Zone::Stack).id();
        let owner = world.spawn_empty().id();
        let hand_entity = world.spawn((zones::Zone::Hand, cards::Owner(owner))).id();
        let mut state =
            SystemState::<Query<(Entity, &zones::Zone, Option<&cards::Owner>)>>::new(&mut world);
        let zone_entities = state.get(&world).unwrap();
        assert_eq!(
            find_entity(zones::Zone::Stack, None, &zone_entities),
            Some(stack_entity)
        );
        assert_eq!(
            find_entity(zones::Zone::Hand, Some(owner), &zone_entities),
            Some(hand_entity)
        );
        assert_eq!(
            find_entity(zones::Zone::Graveyard, Some(owner), &zone_entities),
            None
        );
    }
}
