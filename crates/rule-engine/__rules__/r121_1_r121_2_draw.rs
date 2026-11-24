use crate::{HAND, LIBRARY};
use bevy::prelude::{Commands, Entity, MessageReader, MessageWriter, Query};

use crate::Attempt;
use crate::Resolve;
use crate::rules;
use crate::zones;

/// 121.1. A player draws a card by putting the top card of their library into their hand. This is
/// done as a turn-based action during each player’s draw step. It may also be done as part of a
/// cost or effect of a spell or ability.
///
/// 121.2. Cards may only be drawn one at a time. If a player is instructed to draw multiple cards,
/// that player performs that many individual card draws.
///
/// # Notes
///
/// The top card of a library is the first card in its `ZoneContents`, and `MockGameBuilder` lists a
/// library top first.
pub fn r121_1_r121_2_draw(
    mut draws: MessageReader<Resolve<rules::Draw>>,
    mut attempted: MessageWriter<Attempt<rules::DrawFromEmptyLibrary>>,
    mut commands: Commands,
    zones: zones::ZoneLookup,
    zone_contents: Query<&zones::ZoneContents>,
) {
    for draw in draws.read() {
        let player = draw.player;
        let count = draw.amount;
        let Some(library) = zones.find(LIBRARY, Some(player)) else {
            continue;
        };
        let drawn_from_top: Vec<Entity> = zone_contents
            .get(library)
            .map(|contents| {
                let requested = usize::try_from(count).unwrap_or(usize::MAX);
                contents.iter().take(requested).copied().collect()
            })
            .unwrap_or_default();
        if let Some(hand) = zones.find(HAND, Some(player)) {
            for card in &drawn_from_top {
                commands.entity(*card).insert(zones::InZone(hand));
            }
        }
        if u32::try_from(drawn_from_top.len()).unwrap_or(u32::MAX) < count {
            attempted.write(Attempt(rules::DrawFromEmptyLibrary(player)));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mana;
    use crate::testing::*;
    use crate::turns;
    use crate::{GRAVEYARD, HAND, LIBRARY};

    #[test]
    fn divination_draws_the_top_two_cards_of_the_library() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([DIVINATION])
                .library([GRIZZLY_BEARS, LIGHTNING_BOLT, MURDER])
                .mana_pool(mana![2, U]),
            P2,
        ]);
        let [divination] = game.player(p1).cards_in(HAND).exactly::<1>(DIVINATION)?;
        game.play([p1.cast(divination).pass_priority(), p2.pass_priority()])?;
        assert!(game.player(p1).cards_in(HAND).has(GRIZZLY_BEARS));
        assert!(game.player(p1).cards_in(HAND).has(LIGHTNING_BOLT));
        assert!(game.player(p1).cards_in(GRAVEYARD).has(divination));
        game.player(p1).cards_in(LIBRARY).exactly::<1>(MURDER)?;
        Ok(())
    }

    #[test]
    fn sign_in_blood_draw_two_is_replaced_by_lose_two_with_painful_lessons() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([SIGN_IN_BLOOD])
                .library([LIGHTNING_BOLT, MURDER])
                .battlefield([TEST_PAINFUL_LESSONS])
                .mana_pool(mana![B, B]),
            P2,
        ]);
        let [sign_in_blood] = game.player(p1).cards_in(HAND).exactly::<1>(SIGN_IN_BLOOD)?;
        game.play([
            p1.cast(sign_in_blood).targeting(p1).end_turn(),
            p2.end_turn(),
        ])?;
        assert!(!game.player(p1).cards_in(HAND).has(LIGHTNING_BOLT));
        assert!(!game.player(p1).cards_in(HAND).has(MURDER));
        assert_eq!(game.life(p1), 16);
        Ok(())
    }

    #[test]
    fn draw_step_draw_one_is_not_replaced_with_painful_lessons() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::builder().turn(2, turns::Step::Upkeep).players([
            P1.battlefield([TEST_PAINFUL_LESSONS]).library([GRIZZLY_BEARS, LIGHTNING_BOLT]),
            P2,
        ]);
        let [bears] = game.player(p1).cards_in(LIBRARY).exactly::<1>(GRIZZLY_BEARS)?;
        let hand_before = game.player(p1).cards_in(HAND).count();

        game.play([p1.pass_priority(), p2.pass_priority(), p1.pass_priority()])?;

        assert!(game.player(p1).cards_in(HAND).has(bears));
        assert_eq!(game.player(p1).cards_in(HAND).count(), hand_before + 1);
        assert_eq!(game.life(p1), 20);
        Ok(())
    }
}
