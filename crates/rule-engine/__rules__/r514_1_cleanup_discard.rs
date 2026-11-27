use bevy::ecs::schedule::{IntoScheduleConfigs, ScheduleConfigs};
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::MessageWriter;

use crate::Attempt;
use crate::Game;
use crate::events;
use crate::rules;
use crate::turns;

/// 514.1. First, if the active player’s hand contains more cards than their maximum hand size
/// (normally seven), they discard enough cards to reduce their hand size to that number. This
/// turn-based action doesn’t use the stack.
pub fn r514_1_cleanup_discard() -> ScheduleConfigs<ScheduleSystem> {
    discard_to_maximum_hand_size.run_if(turns::Step::Cleanup.begins())
}

fn discard_to_maximum_hand_size(
    mut discards: MessageWriter<Attempt<events::Discard>>,
    game: Game,
    answers: rules::Rules,
) {
    let Some(active_player) = game.active_player() else {
        return;
    };
    let maximum_hand_size = answers.value::<rules::MaximumHandSize>(*active_player.id);
    let excess = maximum_hand_size.excess(active_player.hand().len());
    if excess > 0 {
        discards.write(Attempt(events::Discard {
            player: active_player.id,
            amount: u32::try_from(excess).unwrap_or(u32::MAX),
        }));
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;
    use crate::turns;
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
        assert_eq!(game.turn().number, 1);
        assert_eq!(game.turn().step, turns::Step::Cleanup);
        game.play([p1.discard([bears])])?;
        assert_eq!(game.turn().number, 2);
        assert_eq!(game.turn().active, p2.id);
        assert!(game.card(bears).is_in(GRAVEYARD).of(p1));
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
