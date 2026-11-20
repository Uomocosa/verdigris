use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::{Component, Entity, Query};

use crate::cards;
use crate::effects;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Zone {
    Library,
    Hand,
    Battlefield,
    Graveyard,
    Exile,
    Stack,
}

#[rustfmt::skip]
impl Zone {
    #[must_use]
    pub fn of(self, player: effects::PlayerRef) -> effects::ZoneRef { effects::ZoneRef::from(self).of(player) }

    #[must_use]
    pub fn contains_card(
        self,
        card_zone: Entity,
        zone_entities: &Query<(Entity, &Self, Option<&cards::Owner>)>,
    ) -> bool { zone_entities.get(card_zone).is_ok_and(|(_, found_zone, _)| *found_zone == self) }
}

#[atomic_delegates]
impl Zone {
    #[must_use]
    pub fn find_entity(
        self,
        owner: Option<Entity>,
        zone_entities: &Query<(Entity, &Self, Option<&cards::Owner>)>,
    ) -> Option<Entity> {
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::SystemState;
    use bevy::prelude::{Entity, Query, World};

    use crate::cards;
    use crate::effects;
    use crate::zones;
    use crate::{BATTLEFIELD, YOU};

    use super::Zone;

    #[test]
    fn test_usage() {
        assert_eq!(
            BATTLEFIELD.of(YOU),
            effects::ZoneRef {
                zone: Zone::Battlefield,
                of: effects::PlayerRef::You,
            }
        );

        let mut world = World::new();
        let card = world.spawn(zones::Zone::Battlefield).id();
        let mut state =
            SystemState::<Query<(Entity, &zones::Zone, Option<&cards::Owner>)>>::new(&mut world);
        let zone_entities = state.get(&world).unwrap();
        assert!(Zone::Battlefield.contains_card(card, &zone_entities));
        assert!(!Zone::Hand.contains_card(card, &zone_entities));
    }
}
