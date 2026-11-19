use bevy::prelude::Entity;

use crate::testing;
use crate::zones;

pub fn find(
    game: &testing::MockGame,
    player: Entity,
    zone: zones::Zone,
    name: &str,
) -> Option<Entity> {
    game.entities_in_zone_matching_card(player, zone, name)
        .into_iter()
        .next()
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::testing::*;

    use super::find;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::new([P1.hand([LIGHTNING_BOLT]), P2]);
        let found = find(&game, p1, HAND, "Lightning Bolt");
        assert!(found.is_some());
        assert!(find(&game, p1, HAND, "Murder").is_none());
    }
}
