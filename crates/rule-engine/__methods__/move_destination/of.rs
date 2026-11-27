use bevy::prelude::Entity;

use crate::testing;
use crate::zones;

pub fn of(destination: testing::MoveDestination<'_>, player: testing::TestPlayer) -> bool {
    let mut current = Entity::from(destination.card);
    loop {
        let Some(left) = destination.game.get::<zones::LeftZone>(current) else {
            return current != Entity::from(destination.card)
                && destination
                    .game
                    .cards_in_zone(player.id, destination.zone)
                    .iter()
                    .any(|object| {
                        matches!(object, testing::ZoneObject::Card { card, .. } if Entity::from(*card) == current)
                    });
        };
        current = *left;
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::GRAVEYARD;
    use crate::testing::*;

    use super::of;

    #[test]
    fn test_usage() -> TestResult {
        let (game, [p1, ..]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let destination = game.moved(bears).to(GRAVEYARD);

        assert!(!of(destination, p1));
        Ok(())
    }
}
