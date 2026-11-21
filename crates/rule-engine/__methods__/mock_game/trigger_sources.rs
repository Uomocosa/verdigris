use bevy::prelude::Entity;

use crate::stack;
use crate::testing;
use crate::zones;

pub fn trigger_sources(game: &testing::MockGame) -> Vec<Entity> {
    let world = game.app.world();
    world
        .get::<zones::ZoneContents>(game.stack)
        .map(|contents| {
            contents
                .iter()
                .filter_map(|entity| {
                    world
                        .get::<stack::TriggeredAbilityOnStack>(*entity)
                        .map(|ability| ability.source)
                })
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use crate::testing::*;

    use super::trigger_sources;

    #[test]
    fn test_usage() {
        let (game, _) = MockGame::new([P1, P2]);
        assert_eq!(trigger_sources(&game).len(), 0);
    }
}
