use atomic_delegate_macros::atomic_delegates;

use crate::testing;
use crate::zones;

#[derive(Clone, Copy)]
pub struct MoveDestination<'g> {
    pub(crate) game: &'g testing::MockGame,
    pub(crate) card: testing::TestCard,
    pub(crate) zone: zones::Zone,
}

impl<'g> MoveDestination<'g> {
    pub(crate) const fn from_game_card_and_zone(
        game: &'g testing::MockGame,
        card: testing::TestCard,
        zone: zones::Zone,
    ) -> Self {
        Self { game, card, zone }
    }
}

#[atomic_delegates]
impl MoveDestination<'_> {
    #[must_use]
    pub fn of(self, player: testing::TestPlayer) -> bool {}
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::{Commands, Entity};

    use crate::BATTLEFIELD;
    use crate::GRAVEYARD;
    use crate::cards;
    use crate::testing::*;
    use crate::zones;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let graveyard = game.zone_entity(p1, GRAVEYARD).unwrap();
        let definition = game.get::<cards::CardDefinition>(*bears).unwrap();
        let owner = game.get::<cards::Owner>(*bears).unwrap();
        let old = Entity::from(bears);
        game.app
            .world_mut()
            .run_system_once(move |mut commands: Commands| {
                zones::move_object(
                    &mut commands,
                    old,
                    definition.clone(),
                    owner,
                    None,
                    graveyard,
                );
            })
            .unwrap();

        assert!(game.moved(bears).to(GRAVEYARD).of(p1));
        assert!(!game.moved(bears).to(GRAVEYARD).of(p2));
        Ok(())
    }
}
