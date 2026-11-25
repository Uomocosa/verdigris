use bevy::prelude::Resource;
use derive_more::Deref;

use crate::actions;
use crate::players;

#[derive(Resource, Debug, Default, Deref)]
pub struct PendingChoice(pub Option<actions::Choice>);

impl PendingChoice {
    pub(crate) fn refresh(&mut self, queue: &actions::AskQueue, players: &players::Players) {
        let Self(pending) = self;
        if pending.as_ref().is_some_and(|choice| !players.is_still_in_the_game(choice.player())) {
            *pending = None;
        }
        if pending.is_none() {
            *pending = actions::Choice::first_queued(queue);
        }
    }

    #[must_use]
    pub(crate) fn blocks_priority(&self) -> bool {
        let Self(pending) = self;
        pending.as_ref().is_some_and(actions::Choice::blocks_priority)
    }
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::actions;
    use crate::testing::*;

    #[test]
    fn test_usage() {
        let (game, [p1, ..]) = MockGame::new([P1, P2]);
        let pending = actions::PendingChoice(None);
        assert!(pending.is_none());
        assert!(game.player(p1).cards_in(HAND).has_no(GRIZZLY_BEARS));
    }
}
