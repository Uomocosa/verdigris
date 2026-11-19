use bevy::prelude::Messages;

use crate::actions;
use crate::testing;

pub fn last_rejection(game: &testing::MockGame) -> Option<&actions::Rejected> {
    let messages = game
        .app
        .world()
        .get_resource::<Messages<actions::Rejected>>()?;
    let last = messages
        .oldest_message_count()
        .checked_add(messages.len())?
        .checked_sub(1)?;
    messages.get_message(last).map(|(message, _id)| message)
}

#[cfg(test)]
mod tests {
    use crate::testing::*;

    use super::last_rejection;

    #[test]
    fn test_usage() {
        let game = MockGame::builder().build();
        assert!(last_rejection(&game).is_none());
    }
}
