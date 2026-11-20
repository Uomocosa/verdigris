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
    use bevy::prelude::Entity;

    use crate::cards;
    use crate::effects;
    use crate::testing::*;
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

        let (mut game, [p1, ..]) = MockGame::new([P1, P2]);
        let battlefield = game.zone_entity(p1, BATTLEFIELD).unwrap();
        let (battlefield_holds, hand_holds) = game
            .with_query::<(Entity, &zones::Zone, Option<&cards::Owner>), (bool, bool)>(
                |zone_entities| {
                    (
                        Zone::Battlefield.contains_card(battlefield, &zone_entities),
                        Zone::Hand.contains_card(battlefield, &zone_entities),
                    )
                },
            );
        assert!(battlefield_holds);
        assert!(!hand_holds);
    }
}
