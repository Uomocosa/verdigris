use crate::{GRAVEYARD, HAND};
use std::collections::BTreeSet;

use bevy::prelude::{Commands, Entity, MessageReader, MessageWriter, Query, ResMut};

use crate::actions;
use crate::players;
use crate::zones;

/// 701.9a To discard a card, move it from its owner’s hand to that player’s graveyard.
///
/// 701.9b By default, effects that cause a player to discard a card allow the affected player to
/// choose which card to discard. Some effects, however, require a random discard or allow another
/// player to choose which card is discarded.
pub fn r701_9a_r701_9b_discard(
    mut discards: MessageReader<players::DiscardRequested>,
    mut actions: MessageReader<actions::PlayerAction>,
    mut rejected: MessageWriter<actions::Rejected>,
    mut pending: ResMut<actions::PendingChoice>,
    mut commands: Commands,
    zones: zones::ZoneLookup,
    zone_contents: Query<&zones::ZoneContents>,
) {
    for discard in discards.read() {
        let Some(hand) = zones.find(HAND, Some(discard.player)) else {
            continue;
        };
        let Some(graveyard) = zones.find(GRAVEYARD, Some(discard.player)) else {
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
        let Some(graveyard) = zones.find(GRAVEYARD, Some(choice.player)) else {
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
    use crate::mana;
    use crate::testing::*;
    use crate::{GRAVEYARD, HAND};

    #[test]
    fn mind_rot_makes_the_target_choose_two_cards_to_discard() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([MIND_ROT]).mana_pool(mana![2, B]),
            P2.hand([GRIZZLY_BEARS, LIGHTNING_BOLT, MURDER]),
        ]);
        let [mind_rot] = game.player(p1).cards_in(HAND).exactly::<1>(MIND_ROT)?;
        let [bears] = game.player(p2).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;
        let [bolt] = game.player(p2).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        game.play([p1.cast(mind_rot).targeting(p2).end_turn(), p2.end_turn()])?;

        game.play([p2.discard([bears, bolt])])?;

        assert!(game.player(p2).cards_in(GRAVEYARD).has(bears));
        assert!(game.player(p2).cards_in(GRAVEYARD).has(bolt));
        assert!(game.player(p2).cards_in(HAND).has(MURDER));
        Ok(())
    }

    #[test]
    fn mind_rot_discards_every_card_when_the_hand_holds_fewer_than_two() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([MIND_ROT]).mana_pool(mana![2, B]),
            P2.hand([GRIZZLY_BEARS]),
        ]);
        let [mind_rot] = game.player(p1).cards_in(HAND).exactly::<1>(MIND_ROT)?;
        game.play([p1.cast(mind_rot).targeting(p2).end_turn(), p2.end_turn()])?;

        assert!(game.player(p2).cards_in(GRAVEYARD).has(GRIZZLY_BEARS));
        assert!(game.player(p2).cards_in(HAND).has_no(GRIZZLY_BEARS));
        Ok(())
    }
}
