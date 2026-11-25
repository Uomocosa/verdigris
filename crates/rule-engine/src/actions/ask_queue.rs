use bevy::prelude::{Messages, Resource};
use derive_more::{Deref, DerefMut};

use crate::actions;
use crate::players;

#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub struct AskQueue(pub Vec<actions::Ask>);

impl AskQueue {
    pub(crate) fn ingest(&mut self, asks: &mut Messages<actions::Ask>, players: &players::Players) {
        let Self(queued) = self;
        for ask in asks.drain() {
            let asked: &actions::Choice = &ask;
            if asked.is_found_by_a_state_based_check() && queued.iter().any(|queued| queued == &ask)
            {
                continue;
            }
            queued.push(ask);
        }
        queued.retain(|ask| ask.player().is_none_or(|player| players.is_still_in_the_game(player)));
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
        let queue = actions::AskQueue::default();
        assert!(queue.is_empty());
        assert!(game.player(p1).cards_in(HAND).has_no(GRIZZLY_BEARS));
    }
}
