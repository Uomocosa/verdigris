use bevy::prelude::Messages;

use crate::actions;
use crate::testing;

pub fn rejection_count(game: &testing::MockGame) -> usize {
    let messages = game
        .app
        .world()
        .get_resource::<Messages<actions::Rejected>>();
    messages.map_or(0, |messages| {
        messages
            .oldest_message_count()
            .saturating_add(messages.len())
    })
}

#[cfg(test)]
mod tests {
    use crate::testing::*;

    use super::rejection_count;

    #[test]
    fn test_usage() {
        let (game, _) = MockGame::new([P1, P2]);

        assert_eq!(rejection_count(&game), 0);
    }
}
