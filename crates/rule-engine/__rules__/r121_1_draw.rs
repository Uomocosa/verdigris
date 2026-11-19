use bevy::prelude::{Commands, Entity, MessageReader, Query};

use crate::rules::helpers;
use crate::zones;

/// 121.1 A player draws a card by putting the top card of their library into their hand. This is
/// done as a turn-based action during each player's draw step. It may also be done as part of a
/// cost or effect of a spell or ability.
///
/// 121.2 Cards may only be drawn one at a time. If a player is instructed to draw multiple cards,
/// that player performs that many individual card draws.
///
/// The top card of a library is the first card in its `ZoneContents`, and `MockGameBuilder` lists
/// a library top first.
pub fn r121_1_draw(
    mut draws: MessageReader<zones::DrawRequested>,
    mut commands: Commands,
    zone_entities: Query<(Entity, &zones::Zone, Option<&zones::Owner>)>,
    zone_contents: Query<&zones::ZoneContents>,
) {
    for draw in draws.read() {
        let Some(library) =
            helpers::find_zone(zones::Zone::Library, Some(draw.player), &zone_entities)
        else {
            continue;
        };
        let drawn_from_top: Vec<Entity> = zone_contents
            .get(library)
            .map(|contents| {
                let requested = usize::try_from(draw.count).unwrap_or(usize::MAX);
                contents.iter().take(requested).copied().collect()
            })
            .unwrap_or_default();
        if let Some(hand) = helpers::find_zone(zones::Zone::Hand, Some(draw.player), &zone_entities)
        {
            for card in &drawn_from_top {
                commands.entity(*card).insert(zones::InZone(hand));
            }
        }
        if u32::try_from(drawn_from_top.len()).unwrap_or(u32::MAX) < draw.count {
            commands
                .entity(draw.player)
                .insert(zones::AttemptedDrawFromEmptyLibrary);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::actions;
    use crate::mana;
    use crate::testing::*;
    use crate::zones;

    #[test]
    fn test_usage() {
        let mut game = MockGame::builder()
            .hand(Seat::P1, [divination()])
            .mana_pool(Seat::P1, mana![2, U])
            .build();
        let divination = game.find(game.p1, zones::Zone::Hand, "Divination").unwrap();
        game.act(game.p1, actions::Action::Cast(divination));
        game.pass_priority_all();
        assert!(
            game.find(game.p1, zones::Zone::Hand, "Grizzly Bears")
                .is_some()
        );
        assert_eq!(game.zone_of(divination), zones::Zone::Graveyard);
    }

    #[test]
    fn divination_draws_the_top_two_cards_of_the_library() {
        let mut game = MockGame::builder()
            .hand(Seat::P1, [divination()])
            .library(Seat::P1, [grizzly_bears(), lightning_bolt(), murder()])
            .mana_pool(Seat::P1, mana![2, U])
            .build();
        let divination = game.find(game.p1, zones::Zone::Hand, "Divination").unwrap();
        game.act(game.p1, actions::Action::Cast(divination));
        game.pass_priority_all();
        assert!(
            game.find(game.p1, zones::Zone::Hand, "Grizzly Bears")
                .is_some()
        );
        assert!(
            game.find(game.p1, zones::Zone::Hand, "Lightning Bolt")
                .is_some()
        );
        assert_eq!(
            game.zone_of(game.find(game.p1, zones::Zone::Library, "Murder").unwrap()),
            zones::Zone::Library
        );
    }
}
