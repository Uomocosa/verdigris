use crate::{HAND, LIBRARY};
use bevy::prelude::{Commands, Entity, MessageReader, Query};

use crate::cards;
use crate::players;
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
    mut draws: MessageReader<players::DrawRequested>,
    mut commands: Commands,
    zone_entities: Query<(Entity, &zones::Zone, Option<&cards::Owner>)>,
    zone_contents: Query<&zones::ZoneContents>,
) {
    for draw in draws.read() {
        let Some(library) = LIBRARY.find_entity(Some(draw.player), &zone_entities) else {
            continue;
        };
        let drawn_from_top: Vec<Entity> = zone_contents
            .get(library)
            .map(|contents| {
                let requested = usize::try_from(draw.count).unwrap_or(usize::MAX);
                contents.iter().take(requested).copied().collect()
            })
            .unwrap_or_default();
        if let Some(hand) = HAND.find_entity(Some(draw.player), &zone_entities) {
            for card in &drawn_from_top {
                commands.entity(*card).insert(zones::InZone(hand));
            }
        }
        if u32::try_from(drawn_from_top.len()).unwrap_or(u32::MAX) < draw.count {
            commands.entity(draw.player).insert(players::AttemptedDrawFromEmptyLibrary);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mana;
    use crate::testing::*;
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
        assert_eq!(game.zone_of(divination), GRAVEYARD);
        assert_eq!(
            game.zone_of(game.player(p1).cards_in(LIBRARY).exactly::<1>(MURDER)?[0]),
            LIBRARY
        );
        Ok(())
    }
}
