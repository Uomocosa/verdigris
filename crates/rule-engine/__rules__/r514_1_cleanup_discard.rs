use bevy::prelude::{MessageReader, MessageWriter, Query, Res};

use crate::HAND;
use crate::rules;
use crate::turns;
use crate::zones;

/// 514.1. First, if the active player’s hand contains more cards than their maximum hand size
/// (normally seven), they discard enough cards to reduce their hand size to that number. This
/// turn-based action doesn’t use the stack.
pub fn r514_1_cleanup_discard(
    mut step_started: MessageReader<turns::StepStarted>,
    mut discards: MessageWriter<rules::Would<rules::Discard>>,
    turn: Res<turns::Turn>,
    answers: rules::Rules,
    zones: zones::ZoneLookup,
    zone_contents: Query<&zones::ZoneContents>,
) {
    let turn = turn.into_inner();
    for step in step_started.read() {
        if **step != turns::Step::Cleanup {
            continue;
        }
        let Some(hand) = zones.find(HAND, Some(*turn.active)) else {
            continue;
        };
        let Ok(contents) = zone_contents.get(hand) else {
            continue;
        };
        let excess = answers.value::<rules::MaximumHandSize>(*turn.active).excess(contents.len());
        if excess == 0 {
            continue;
        }
        discards.write(rules::Would(rules::Discard {
            player: turn.active,
            amount: u32::try_from(excess).unwrap_or(u32::MAX),
        }));
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;
    use crate::{GRAVEYARD, HAND};

    #[test]
    fn cleanup_with_empty_hand_discards_nothing() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1, P2]);
        game.play([p1.end_turn(), p2.end_turn()])?;

        assert_eq!(game.player(p1).cards_in(HAND).count(), 0);
        Ok(())
    }

    #[test]
    fn active_player_discards_excess_cards_in_cleanup() -> TestResult {
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
        let [bears] = game.player(p1).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([p1.end_turn(), p2.end_turn()])?;
        game.play([p1.discard([bears])])?;
        assert!(game.player(p1).cards_in(GRAVEYARD).has(bears));
        assert_eq!(game.player(p1).cards_in(HAND).count(), 7);
        Ok(())
    }

    #[test]
    fn reliquary_tower_removes_the_cleanup_discard() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([RELIQUARY_TOWER]).hand([
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
        game.play([p1.end_turn(), p2.end_turn()])?;

        assert_eq!(game.player(p1).cards_in(HAND).count(), 8);
        Ok(())
    }
}
