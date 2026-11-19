use bevy::prelude::Entity;
use derive_more::Deref;

use crate::testing;
use crate::turns;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deref)]
pub struct TestPlayer {
    #[deref]
    pub(crate) entity: Entity,
    pub(crate) seat: testing::PlayerSeat,
}

impl TestPlayer {
    #[must_use]
    pub(crate) const fn from_entity_and_seat(entity: Entity, seat: testing::PlayerSeat) -> Self {
        Self { entity, seat }
    }
}

#[rustfmt::skip]
impl From<TestPlayer> for Entity {
    fn from(player: TestPlayer) -> Self { player.entity }
}

#[rustfmt::skip]
impl PartialEq<TestPlayer> for Entity {
    fn eq(&self, other: &TestPlayer) -> bool { *self == **other }
}

#[rustfmt::skip]
impl TestPlayer {
    #[must_use]
    pub fn cast(self, spell: Entity) -> testing::TestPlayerActions { testing::TestPlayerActions::from_cast(self, spell) }

    #[must_use]
    pub fn pass_priority(self) -> testing::TestPlayerActions { testing::TestPlayerActions::of(self).pass_priority() }

    #[must_use]
    pub fn pass_to(self, step: turns::Step) -> testing::TestPlayerActions { testing::TestPlayerActions::of(self).pass_to(step) }

    #[must_use]
    pub fn end_current_step(self) -> testing::TestPlayerActions { testing::TestPlayerActions::of(self).end_current_step() }

    #[must_use]
    pub fn end_phase(self) -> testing::TestPlayerActions { testing::TestPlayerActions::of(self).end_phase() }

    #[must_use]
    pub fn end_turn(self) -> testing::TestPlayerActions { testing::TestPlayerActions::of(self).end_turn() }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{Entity, World};

    use crate::testing;
    use crate::turns;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let entity = world.spawn_empty().id();
        let spell = world.spawn_empty().id();
        let player = testing::TestPlayer::from_entity_and_seat(entity, testing::PlayerSeat(1));

        assert_eq!(Entity::from(player), entity);
        assert_eq!(*player, entity);
        assert_eq!(*player.seat, 1);
        assert_eq!(player.cast(spell).steps.len(), 1);
        assert_eq!(
            player.pass_to(turns::Step::PostcombatMain).steps,
            vec![testing::PlayStep::AutoPass {
                player: entity,
                until: testing::AutoPassUntil::StepStarts(turns::Step::PostcombatMain),
            }]
        );
    }
}
