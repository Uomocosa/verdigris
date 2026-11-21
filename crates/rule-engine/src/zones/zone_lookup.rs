use atomic_delegate_macros::atomic_delegates;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Entity, Query};
use derive_more::Deref;

use crate::cards;
use crate::zones;

#[derive(Clone, Copy, SystemParam, Deref)]
pub struct ZoneLookup<'w, 's>(
    Query<'w, 's, (Entity, &'static zones::Zone, Option<&'static cards::Owner>)>,
);

#[rustfmt::skip]
impl ZoneLookup<'_, '_> {
    #[must_use]
    pub fn contains(&self, zone: impl Into<zones::Zone>, card_zone: Entity) -> bool { self.get(card_zone).is_ok_and(|(_, found_zone, _)| *found_zone == zone.into()) }
}

#[atomic_delegates]
impl ZoneLookup<'_, '_> {
    #[must_use]
    pub fn find(&self, zone: impl Into<zones::Zone>, owner: Option<Entity>) -> Option<Entity> {}
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::testing::*;
    use crate::{BATTLEFIELD, HAND};

    use super::ZoneLookup;
    use crate::zones;

    #[test]
    fn test_usage() {
        let (mut game, [p1, ..]) = MockGame::new([P1, P2]);
        let battlefield = game.zone_entity(p1, BATTLEFIELD).unwrap();
        let hand = game.zone_entity(p1, HAND).unwrap();
        game.app
            .world_mut()
            .run_system_once(move |zones: ZoneLookup| {
                assert!(zones.contains(zones::Zone::Battlefield, battlefield));
                assert!(!zones.contains(zones::Zone::Hand, battlefield));
                assert_eq!(zones.find(zones::Zone::Hand, Some(p1.into())), Some(hand));
                assert_eq!(
                    zones.find(zones::Zone::Battlefield, Some(p1.into())),
                    Some(battlefield)
                );
            })
            .unwrap();
    }
}
