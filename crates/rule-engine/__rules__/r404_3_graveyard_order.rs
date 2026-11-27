use std::collections::BTreeMap;

use bevy::ecs::schedule::{IntoScheduleConfigs, ScheduleConfigs};
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::{Added, Changed, Entity, MessageReader, MessageWriter, Or, Query};

use crate::GRAVEYARD;
use crate::GRAVEYARD_ORDER;
use crate::actions;
use crate::cards;
use crate::players;
use crate::zones;

type MovedToGraveyard<'w, 's> = Query<
    'w,
    's,
    (Entity, &'static cards::Owner, &'static zones::InZone),
    Or<(Added<zones::InZone>, Changed<zones::InZone>)>,
>;

/// 404.3. If an effect or rule puts two or more cards into the same graveyard at the same time,
/// the owner of those cards may arrange them in any order.
pub fn r404_3_graveyard_order() -> ScheduleConfigs<ScheduleSystem> {
    (arrange_as_answered, ask_for_the_order).chain()
}

fn arrange_as_answered(
    mut answers: MessageReader<actions::Answered>,
    mut zone_contents: Query<&mut zones::ZoneContents>,
) {
    for answered in answers.read() {
        let actions::Choice::Pick(choice) = &answered.choice else {
            continue;
        };
        if choice.id != GRAVEYARD_ORDER {
            continue;
        }
        let actions::Action::Choose(ordered) = &answered.answer else {
            continue;
        };
        let Some(graveyard) = choice.source else {
            continue;
        };
        let Ok(mut contents) = zone_contents.get_mut(graveyard) else {
            continue;
        };
        let Some(expected) = choice.fixed() else {
            continue;
        };
        let mut ordered = ordered.iter().copied();
        for card in contents.iter_mut() {
            if expected.contains(card) {
                let Some(next) = ordered.next() else {
                    break;
                };
                *card = next;
            }
        }
    }
}

fn ask_for_the_order(
    mut asks: MessageWriter<actions::Ask>,
    moved: MovedToGraveyard,
    zone_contents: Query<&zones::ZoneContents>,
    zones: zones::ZoneLookup,
) {
    let mut moved_cards_by_graveyard: BTreeMap<Entity, (players::PlayerId, Vec<Entity>)> =
        BTreeMap::new();
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
        let count = actions::Count::exactly(u32::try_from(batch.len()).unwrap_or(u32::MAX));
        asks.write(actions::Ask(actions::Choice::Pick(actions::PickChoice {
            id: GRAVEYARD_ORDER,
            player: owner_entity,
            source: Some(graveyard),
            among: actions::Candidates::Fixed(batch.into_iter().collect()),
            count,
            ordered: true,
        })));
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
        let [_bears] = game.player(p2).cards_in(LIBRARY).exactly::<1>(GRIZZLY_BEARS)?;
        let [_corpse] = game.player(p2).cards_in(LIBRARY).exactly::<1>(WALKING_CORPSE)?;
        let [_wolf] = game.player(p2).cards_in(LIBRARY).exactly::<1>(WATCHWOLF)?;
        let [_rats] = game.player(p2).cards_in(LIBRARY).exactly::<1>(TYPHOID_RATS)?;
        let [_drake] = game.player(p2).cards_in(LIBRARY).exactly::<1>(WIND_DRAKE)?;
        let [forest] = game.player(p2).cards_in(LIBRARY).exactly::<1>(FOREST)?;

        game.play([
            p1.cast(tome_scour).targeting(p2).pass_priority(),
            p2.pass_priority(),
        ])?;
        let [bears_buried] = game.player(p2).cards_in(GRAVEYARD).exactly::<1>(GRIZZLY_BEARS)?;
        let [corpse_buried] = game.player(p2).cards_in(GRAVEYARD).exactly::<1>(WALKING_CORPSE)?;
        let [wolf_buried] = game.player(p2).cards_in(GRAVEYARD).exactly::<1>(WATCHWOLF)?;
        let [rats_buried] = game.player(p2).cards_in(GRAVEYARD).exactly::<1>(TYPHOID_RATS)?;
        let [drake_buried] = game.player(p2).cards_in(GRAVEYARD).exactly::<1>(WIND_DRAKE)?;
        game.play([p2.arrange_graveyard([
            drake_buried,
            wolf_buried,
            rats_buried,
            corpse_buried,
            bears_buried,
        ])])?;

        let graveyard = game.zone_entity(p2, GRAVEYARD).expect("p2 graveyard");
        let top_first = game.with_query::<&zones::ZoneContents, Vec<Entity>>(|contents| {
            contents.get(graveyard).map(|cards| cards.iter().copied().collect()).unwrap_or_default()
        });
        assert_eq!(
            top_first,
            vec![
                Entity::from(drake_buried),
                Entity::from(wolf_buried),
                Entity::from(rats_buried),
                Entity::from(corpse_buried),
                Entity::from(bears_buried),
            ]
        );
        assert!(game.card(forest).is_in(LIBRARY).of(p2));
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
