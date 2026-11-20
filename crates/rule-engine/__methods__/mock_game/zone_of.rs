use crate::LIBRARY;
use crate::testing;
use crate::zones;

pub fn zone_of(game: &testing::MockGame, card: impl Into<bevy::prelude::Entity>) -> zones::Zone {
    let card = card.into();
    let world = game.app.world();
    world
        .get::<zones::InZone>(card)
        .and_then(|in_zone| world.get::<zones::Zone>(**in_zone).copied())
        .unwrap_or(LIBRARY)
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::testing::*;

    use super::zone_of;

    #[test]
    fn test_usage() {
        let (game, [_, p2, ..]) = MockGame::new([P1, P2.battlefield([GRIZZLY_BEARS])]);
        let [bears] =
            game.player(p2).cards_in(BATTLEFIELD).matching(GRIZZLY_BEARS).exactly::<1>().unwrap();
        assert_eq!(zone_of(&game, bears), BATTLEFIELD);
    }
}
