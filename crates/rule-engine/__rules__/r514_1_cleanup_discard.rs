use crate::{HAND, MAX_HAND_SIZE};
use bevy::prelude::{Entity, MessageReader, MessageWriter, Query, Res};

use crate::cards;
use crate::players;
use crate::rules::helpers;
use crate::turns;
use crate::zones;

/// 514.1 First, if the active player's hand contains more cards than their maximum hand size
/// (normally seven), they discard enough cards to reduce their hand size to that number. This
/// turn-based action doesn't use the stack.
pub fn r514_1_cleanup_discard(
    mut step_started: MessageReader<turns::StepStarted>,
    mut discards: MessageWriter<players::DiscardRequested>,
    turn: Res<turns::Turn>,
    zone_entities: Query<(Entity, &zones::Zone, Option<&cards::Owner>)>,
    zone_contents: Query<&zones::ZoneContents>,
) {
    let turn = turn.into_inner();
    for step in step_started.read() {
        if **step != turns::Step::Cleanup {
            continue;
        }
        let Some(hand) = helpers::find_zone(HAND, Some(turn.active), &zone_entities) else {
            continue;
        };
        let Ok(contents) = zone_contents.get(hand) else {
            continue;
        };
        let excess = contents.len().saturating_sub(MAX_HAND_SIZE);
        if excess == 0 {
            continue;
        }
        discards.write(players::DiscardRequested {
            player: turn.active,
            count: u32::try_from(excess).unwrap_or(u32::MAX),
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;
    use crate::{GRAVEYARD, HAND};

    #[test]
    fn test_usage() {
        let (mut game, [p1, p2]) = MockGame::new([P1, P2]);
        game.play([p1.end_turn(), p2.end_turn()]).unwrap();

        assert_eq!(game.player(p1).cards_in(HAND).len(), 0);
    }

    #[test]
    fn r514_1_active_player_discards_excess_cards_in_cleanup() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([
                GRIZZLY_BEARS,
                LIGHTNING_BOLT,
                MURDER,
                DIVINATION,
                CANCEL,
                DISPEL,
                MIND_ROT,
                DOOM_BLADE,
            ]),
            P2,
        ]);
        let [bears] = game.player(p1).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS).unwrap();
        game.play([p1.end_turn(), p2.end_turn()]).unwrap();
        game.play([p1.discard([bears])]).unwrap();
        assert_eq!(game.zone_of(bears), GRAVEYARD);
        assert_eq!(game.player(p1).cards_in(HAND).len(), 7);
    }
}
