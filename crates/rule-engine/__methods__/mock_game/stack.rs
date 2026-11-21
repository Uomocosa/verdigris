use crate::testing;
use crate::zones;

pub fn stack(game: &testing::MockGame) -> testing::CardsInZone {
    testing::CardsInZone::from_objects(
        game.stack_objects(),
        zones::Zone::Stack,
        testing::ZoneOwner::Shared,
    )
}

#[cfg(test)]
mod tests {
    use crate::testing::*;

    use super::stack;

    #[test]
    fn test_usage() {
        let (game, [_, _]) = MockGame::new([P1, P2]);
        assert!(stack(&game).is_empty());
    }
}
