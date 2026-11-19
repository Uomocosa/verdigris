use crate::testing;
use crate::zones;

pub fn zone_of(game: &testing::MockGame, card: bevy::prelude::Entity) -> zones::Zone {
    let world = game.app.world();
    world
        .get::<zones::InZone>(card)
        .and_then(|in_zone| world.get::<zones::Zone>(**in_zone).copied())
        .unwrap_or(zones::Zone::Library)
}

#[cfg(test)]
mod tests {
    use crate::testing::*;
    use crate::zones;

    use super::zone_of;

    #[test]
    fn test_usage() {
        let game = MockGame::builder()
            .battlefield(Seat::P2, [grizzly_bears()])
            .build();
        let bears = game
            .find(game.p2, zones::Zone::Battlefield, "Grizzly Bears")
            .unwrap();
        assert_eq!(zone_of(&game, bears), zones::Zone::Battlefield);
    }
}
