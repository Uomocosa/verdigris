use bevy::prelude::Entity;
use derive_more::Deref;

use crate::players;
use crate::testing;
use crate::turns;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deref)]
pub struct TestPlayer {
    #[deref(forward)]
    pub(crate) id: players::PlayerId,
    pub(crate) seat: testing::PlayerSeat,
}

impl TestPlayer {
    #[must_use]
    pub(crate) const fn from_entity_and_seat(entity: Entity, seat: testing::PlayerSeat) -> Self {
        Self {
            id: players::PlayerId(entity),
            seat,
        }
    }
}

#[rustfmt::skip]
impl From<TestPlayer> for Entity {
    fn from(player: TestPlayer) -> Self { *player.id }
}

#[rustfmt::skip]
impl PartialEq<TestPlayer> for Entity {
    fn eq(&self, other: &TestPlayer) -> bool { *self == **other }
}

#[rustfmt::skip]
impl TestPlayer {
    #[must_use]
    pub fn activate_ability(self, ability: testing::TestAbility) -> testing::TestPlayerActions { testing::TestPlayerActions::of(self).activate_ability(ability) }

    #[must_use]
    pub fn cast(self, spell: impl Into<Entity>) -> testing::TestPlayerActions { testing::TestPlayerActions::from_cast(self, spell.into()) }

    #[must_use]
    pub fn pass_priority(self) -> testing::TestPlayerActions { testing::TestPlayerActions::of(self).pass_priority() }

    #[must_use]
    pub fn concede(self) -> testing::TestPlayerActions { testing::TestPlayerActions::of(self).concede() }

    #[must_use]
    pub fn declare_attackers<const N: usize>(self, attackers: [testing::TestCard; N]) -> testing::TestPlayerActions { testing::TestPlayerActions::of(self).declare_attackers(attackers) }

    #[must_use]
    pub fn declare_blockers<const N: usize>(self, blocks: [(testing::TestCard, testing::TestCard); N]) -> testing::TestPlayerActions { testing::TestPlayerActions::of(self).declare_blockers(blocks) }

    #[must_use]
    pub fn play_land(self, land: testing::TestCard) -> testing::TestPlayerActions { testing::TestPlayerActions::of(self).play_land(land) }

    #[must_use]
    pub fn discard<const N: usize>(self, cards: [testing::TestCard; N]) -> testing::TestPlayerActions { testing::TestPlayerActions::of(self).discard(cards) }

    #[must_use]
    pub fn order_triggers<const N: usize>(self, triggers: [testing::TestCard; N]) -> testing::TestPlayerActions { testing::TestPlayerActions::of(self).order_triggers(triggers) }

    #[must_use]
    pub fn arrange_graveyard<const N: usize>(self, cards: [testing::TestCard; N]) -> testing::TestPlayerActions { testing::TestPlayerActions::of(self).arrange_graveyard(cards) }

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

    use crate::players;
    use crate::testing;
    use crate::turns;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let entity = world.spawn_empty().id();
        let player = testing::TestPlayer::from_entity_and_seat(entity, testing::PlayerSeat(1));

        assert_eq!(Entity::from(player), entity);
        assert_eq!(player.id, players::PlayerId(entity));
        assert_eq!(*player, entity);
        assert_eq!(*player.seat, 1);
        assert_eq!(
            player.pass_to(turns::Step::PostcombatMain).steps,
            vec![testing::PlayStep::AutoPass {
                player: entity,
                until: testing::AutoPassUntil::StepStarts(turns::Step::PostcombatMain),
            }]
        );
        assert_eq!(
            player.end_turn().steps,
            vec![testing::PlayStep::AutoPass {
                player: entity,
                until: testing::AutoPassUntil::OwnNextTurn,
            }]
        );
    }
}
