use bevy::prelude::{Entity, Query};

use crate::cards;
use crate::effects;
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

#[must_use]
pub fn effective_power_toughness(
    definition: &cards::CardDefinition,
    modifier: Option<&effects::PowerToughnessModifier>,
) -> Option<cards::PowerToughness> {
    let base = definition.power_toughness?;
    let Some(modifier) = modifier else {
        return Some(base);
    };
    Some(cards::PowerToughness {
        power: base.power.saturating_add(modifier.power),
        toughness: base.toughness.saturating_add(modifier.toughness),
    })
}

#[must_use]
pub fn does_creature_match_filter(
    definition: &cards::CardDefinition,
    filter: effects::Filter,
) -> bool {
    let is_creature = definition.types.contains(&cards::CardType::Creature);
    let has_excluded_color = filter
        .excluded_color
        .is_some_and(|excluded| definition.colors().contains(&excluded));
    is_creature && !has_excluded_color
}

#[must_use]
pub fn does_spell_match_filter(
    definition: &cards::CardDefinition,
    filter: effects::Filter,
) -> bool {
    let is_creature = definition.types.contains(&cards::CardType::Creature);
    let is_instant = definition.types.contains(&cards::CardType::Instant);
    match filter.kind {
        effects::FilterKind::Spell => true,
        effects::FilterKind::CreatureSpell => is_creature,
        effects::FilterKind::NoncreatureSpell => !is_creature,
        effects::FilterKind::InstantSpell => is_instant,
        effects::FilterKind::Creature | effects::FilterKind::AnyTarget => false,
    }
}
