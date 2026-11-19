use std::collections::BTreeSet;

use bevy::prelude::{Commands, Entity, MessageReader, MessageWriter, Query, ResMut};

use crate::actions;
use crate::rules::helpers;
use crate::zones;

/// 701.9a To discard a card, move it from its owner's hand to that player's graveyard.
///
/// 701.9b By default, effects that cause a player to discard a card allow the affected player to
/// choose which card to discard.
pub fn r701_9_discard(
    mut discards: MessageReader<zones::DiscardRequested>,
    mut actions: MessageReader<actions::PlayerAction>,
    mut rejected: MessageWriter<actions::Rejected>,
    mut pending: ResMut<actions::PendingChoice>,
    mut commands: Commands,
    zone_entities: Query<(Entity, &zones::Zone, Option<&zones::Owner>)>,
    zone_contents: Query<&zones::ZoneContents>,
) {
    for discard in discards.read() {
        let Some(hand) =
            helpers::find_zone(zones::Zone::Hand, Some(discard.player), &zone_entities)
        else {
            continue;
        };
        let Some(graveyard) =
            helpers::find_zone(zones::Zone::Graveyard, Some(discard.player), &zone_entities)
        else {
            continue;
        };
        let held: Vec<Entity> = zone_contents
            .get(hand)
            .map(|contents| contents.iter().copied().collect())
            .unwrap_or_default();
        if held.len() <= usize::try_from(discard.count).unwrap_or(usize::MAX) {
            for card in held {
                commands.entity(card).insert(zones::InZone(graveyard));
            }
            continue;
        }
        *pending = actions::PendingChoice(Some(actions::Choice::Discard(actions::DiscardChoice {
            player: discard.player,
            count: discard.count,
            legal: held.into_iter().collect(),
        })));
    }
    for action in actions.read() {
        let actions::Action::ChooseDiscard(cards) = &action.action else {
            continue;
        };
        let Some(actions::Choice::Discard(choice)) = (*pending).as_ref().cloned() else {
            rejected.write(actions::Rejected::InvalidDiscard {
                player: action.player,
            });
            continue;
        };
        let chosen: BTreeSet<Entity> = cards.iter().copied().collect();
        let count_matches = u32::try_from(cards.len()).unwrap_or(u32::MAX) == choice.count;
        let are_unique = chosen.len() == cards.len();
        let is_the_affected_player = action.player == choice.player;
        let are_legal = chosen.is_subset(&choice.legal);
        if !(count_matches && are_unique && is_the_affected_player && are_legal) {
            rejected.write(actions::Rejected::InvalidDiscard {
                player: action.player,
            });
            continue;
        }
        let Some(graveyard) =
            helpers::find_zone(zones::Zone::Graveyard, Some(choice.player), &zone_entities)
        else {
            continue;
        };
        for card in cards {
            commands.entity(*card).insert(zones::InZone(graveyard));
        }
        *pending = actions::PendingChoice(None);
    }
}

#[cfg(test)]
mod tests {
    use crate::actions;
    use crate::mana;
    use crate::testing::*;
    use crate::zones;

    fn cast_and_resolve_mind_rot(game: &mut MockGame) {
        let mind_rot = game.find(game.p1, zones::Zone::Hand, "Mind Rot").unwrap();
        game.act(game.p1, actions::Action::Cast(mind_rot));
        game.act(game.p1, actions::Action::ChooseTargets(vec![game.p2]));
        game.pass_priority_all();
    }

    #[test]
    fn test_usage() {
        let mut game = MockGame::builder()
            .hand(Seat::P1, [mind_rot()])
            .hand(Seat::P2, [grizzly_bears()])
            .mana_pool(Seat::P1, mana![2, B])
            .build();
        cast_and_resolve_mind_rot(&mut game);
        assert!(
            game.find(game.p2, zones::Zone::Graveyard, "Grizzly Bears")
                .is_some()
        );
    }

    #[test]
    fn mind_rot_makes_the_target_choose_two_cards_to_discard() {
        let mut game = MockGame::builder()
            .hand(Seat::P1, [mind_rot()])
            .hand(Seat::P2, [grizzly_bears(), lightning_bolt(), murder()])
            .mana_pool(Seat::P1, mana![2, B])
            .build();
        cast_and_resolve_mind_rot(&mut game);

        let bears = game
            .find(game.p2, zones::Zone::Hand, "Grizzly Bears")
            .unwrap();
        let bolt = game
            .find(game.p2, zones::Zone::Hand, "Lightning Bolt")
            .unwrap();
        game.act(game.p2, actions::Action::ChooseDiscard(vec![bears, bolt]));

        assert_eq!(game.zone_of(bears), zones::Zone::Graveyard);
        assert_eq!(game.zone_of(bolt), zones::Zone::Graveyard);
        assert!(game.find(game.p2, zones::Zone::Hand, "Murder").is_some());
    }

    #[test]
    fn mind_rot_discards_every_card_when_the_hand_holds_fewer_than_two() {
        let mut game = MockGame::builder()
            .hand(Seat::P1, [mind_rot()])
            .hand(Seat::P2, [grizzly_bears()])
            .mana_pool(Seat::P1, mana![2, B])
            .build();
        cast_and_resolve_mind_rot(&mut game);

        assert_eq!(
            game.zone_of(
                game.find(game.p2, zones::Zone::Graveyard, "Grizzly Bears")
                    .unwrap()
            ),
            zones::Zone::Graveyard
        );
        assert!(
            game.find(game.p2, zones::Zone::Hand, "Grizzly Bears")
                .is_none()
        );
    }
}
