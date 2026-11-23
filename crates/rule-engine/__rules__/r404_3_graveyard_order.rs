use std::collections::{BTreeMap, BTreeSet};

use bevy::prelude::{Changed, Entity, MessageReader, MessageWriter, Query, ResMut};

use crate::GRAVEYARD;
use crate::actions;
use crate::cards;
use crate::zones;

/// 404.3. If an effect or rule puts two or more cards into the same graveyard at the same time,
/// the owner of those cards may arrange them in any order.
pub fn r404_3_graveyard_order(
    mut player_actions: MessageReader<actions::PlayerAction>,
    mut rejected: MessageWriter<actions::Rejected>,
    mut pending: ResMut<actions::PendingChoice>,
    mut zone_contents: Query<&mut zones::ZoneContents>,
    moved: Query<(Entity, &cards::Owner, &zones::InZone), Changed<zones::InZone>>,
    zones: zones::ZoneLookup,
) {
    for action in player_actions.read() {
        let actions::Action::ArrangeGraveyard(answered) = &action.action else {
            continue;
        };
        let Some(actions::Choice::ArrangeGraveyard(choice)) = (*pending).as_ref().cloned() else {
            rejected.write(actions::Rejected::InvalidGraveyardOrder {
                player: action.player,
            });
            continue;
        };
        let expected: BTreeSet<Entity> = choice.cards.iter().copied().collect();
        let received: BTreeSet<Entity> = answered.iter().copied().collect();
        let is_chooser = action.player == choice.player;
        let is_permutation = received == expected && answered.len() == choice.cards.len();
        if !(is_chooser && is_permutation) {
            rejected.write(actions::Rejected::InvalidGraveyardOrder {
                player: action.player,
            });
            continue;
        }
        let Ok(mut contents) = zone_contents.get_mut(choice.graveyard) else {
            continue;
        };
        let mut answered = answered.iter().copied();
        for card in contents.iter_mut() {
            if expected.contains(card) {
                let Some(next) = answered.next() else {
                    break;
                };
                *card = next;
            }
        }
        *pending = actions::PendingChoice(None);
    }

    if pending.is_some() {
        return;
    }
    let mut moved_cards_by_graveyard: BTreeMap<Entity, (Entity, Vec<Entity>)> = BTreeMap::new();
    for (card, owner, in_zone) in &moved {
        let graveyard = **in_zone;
        if zones.find(GRAVEYARD, Some(**owner)) != Some(graveyard) {
            continue;
        }
        let (owner_entity, batch) =
            moved_cards_by_graveyard.entry(graveyard).or_insert_with(|| (**owner, Vec::new()));
        *owner_entity = **owner;
        batch.push(card);
    }
    for (graveyard, (owner_entity, moved_cards)) in moved_cards_by_graveyard {
        if moved_cards.len() < 2 {
            continue;
        }
        let Ok(contents) = zone_contents.get(graveyard) else {
            continue;
        };
        let positions: BTreeMap<Entity, usize> =
            contents.iter().copied().enumerate().map(|(position, card)| (card, position)).collect();
        let mut batch: Vec<Entity> = moved_cards;
        batch.sort_by_key(|card| positions.get(card).copied().unwrap_or(usize::MAX));
        *pending = actions::PendingChoice(Some(actions::Choice::ArrangeGraveyard(
            actions::ArrangeGraveyardChoice {
                player: owner_entity,
                graveyard,
                cards: batch,
            },
        )));
        return;
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::GRAVEYARD;
    use crate::LIBRARY;
    use crate::actions;
    use crate::mana;
    use crate::testing::*;
    use crate::zones;

    #[test]
    fn the_owner_arranges_cards_milled_into_one_graveyard_at_the_same_time() -> TestResult {
        use crate::HAND;

        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([TOME_SCOUR]).mana_pool(mana![U]),
            P2.library([
                GRIZZLY_BEARS,
                WALKING_CORPSE,
                WATCHWOLF,
                TYPHOID_RATS,
                WIND_DRAKE,
                FOREST,
            ]),
        ]);
        let [tome_scour] = game.player(p1).cards_in(HAND).exactly::<1>(TOME_SCOUR)?;
        let [bears] = game.player(p2).cards_in(LIBRARY).exactly::<1>(GRIZZLY_BEARS)?;
        let [corpse] = game.player(p2).cards_in(LIBRARY).exactly::<1>(WALKING_CORPSE)?;
        let [wolf] = game.player(p2).cards_in(LIBRARY).exactly::<1>(WATCHWOLF)?;
        let [rats] = game.player(p2).cards_in(LIBRARY).exactly::<1>(TYPHOID_RATS)?;
        let [drake] = game.player(p2).cards_in(LIBRARY).exactly::<1>(WIND_DRAKE)?;
        let [forest] = game.player(p2).cards_in(LIBRARY).exactly::<1>(FOREST)?;

        game.play([
            p1.cast(tome_scour).targeting(p2).pass_priority(),
            p2.pass_priority(),
        ])?;
        game.play([p2.arrange_graveyard([drake, wolf, rats, corpse, bears])])?;

        let graveyard = game.zone_entity(p2, GRAVEYARD).expect("p2 graveyard");
        let top_first = game.with_query::<&zones::ZoneContents, Vec<Entity>>(|contents| {
            contents.get(graveyard).map(|cards| cards.iter().copied().collect()).unwrap_or_default()
        });
        assert_eq!(
            top_first,
            vec![
                Entity::from(drake),
                Entity::from(wolf),
                Entity::from(rats),
                Entity::from(corpse),
                Entity::from(bears),
            ]
        );
        assert!(game.player(p2).cards_in(LIBRARY).has(forest));
        Ok(())
    }

    #[test]
    fn a_single_milled_card_asks_for_no_arrangement() -> TestResult {
        use crate::HAND;

        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([TOME_SCOUR]).mana_pool(mana![U]),
            P2.library([GRIZZLY_BEARS]),
        ]);
        let [tome_scour] = game.player(p1).cards_in(HAND).exactly::<1>(TOME_SCOUR)?;
        game.play([
            p1.cast(tome_scour).targeting(p2).pass_priority(),
            p2.pass_priority(),
        ])?;

        assert_eq!(game.player(p2).cards_in(GRAVEYARD).count(), 1);
        let pending = game.app.world().resource::<actions::PendingChoice>().as_ref().cloned();
        assert_eq!(pending, None);
        Ok(())
    }
}
