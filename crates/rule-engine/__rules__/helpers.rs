use bevy::prelude::{Entity, Query};

use crate::zones;

pub fn find_zone(
    zone: zones::Zone,
    owner: Option<Entity>,
    zone_entities: &Query<(Entity, &zones::Zone, Option<&zones::Owner>)>,
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

#[must_use]
pub fn is_card_in_zone(
    card_zone: Entity,
    zone: zones::Zone,
    zone_entities: &Query<(Entity, &zones::Zone, Option<&zones::Owner>)>,
) -> bool {
    zone_entities
        .get(card_zone)
        .is_ok_and(|(_, found_zone, _)| *found_zone == zone)
}

#[must_use]
pub fn signed_damage(amount: u32) -> i32 {
    i32::try_from(amount).unwrap_or(i32::MAX)
}
