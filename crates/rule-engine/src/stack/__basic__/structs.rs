use bevy::ecs::system::SystemParam;
use bevy::prelude::{Entity, Query};

use crate::abilities;
use crate::cards;
use crate::players;
use crate::stack;
use crate::zones;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PendingTrigger {
    pub source: Entity,
    pub controller: players::PlayerId,
    pub ability: abilities::TriggeredAbility,
}

#[derive(SystemParam, Clone, Copy)]
pub struct ResolutionLookups<'w, 's> {
    pub in_zone: Query<'w, 's, &'static zones::InZone>,
    pub definitions: Query<'w, 's, &'static cards::CardDefinition>,
    pub abilities: Query<'w, 's, &'static stack::TriggeredAbilityOnStack>,
    pub owners: Query<'w, 's, &'static cards::Owner>,
    pub chosen_targets: Query<'w, 's, &'static stack::ChosenTarget>,
    pub lives: Query<'w, 's, (Entity, &'static players::Life)>,
    pub poison: Query<'w, 's, &'static players::PoisonCounters>,
}
