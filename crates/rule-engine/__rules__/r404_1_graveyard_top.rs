use std::collections::{BTreeMap, BTreeSet};

use bevy::prelude::{Added, Changed, Entity, Or, Query};

use crate::GRAVEYARD;
use crate::cards;
use crate::zones;

type MovedToGraveyard<'w, 's> = Query<
    'w,
    's,
    (Entity, &'static cards::Owner, &'static zones::InZone),
    Or<(Added<zones::InZone>, Changed<zones::InZone>)>,
>;

/// 404.1. A player’s graveyard is their discard pile. Any object that’s countered, discarded,
/// destroyed, or sacrificed is put on top of its owner’s graveyard, as is any instant or sorcery
/// spell that’s finished resolving. Each player’s graveyard starts out empty.
pub fn r404_1_graveyard_top(
    moved: MovedToGraveyard,
    mut zone_contents: Query<&mut zones::ZoneContents>,
    zones: zones::ZoneLookup,
) {
    let mut moved_cards_by_graveyard: BTreeMap<Entity, Vec<Entity>> = BTreeMap::new();
    for (card, owner, in_zone) in &moved {
        let graveyard = **in_zone;
        if zones.find(GRAVEYARD, Some(**owner)) != Some(graveyard) {
            continue;
        }
        moved_cards_by_graveyard.entry(graveyard).or_default().push(card);
    }
    for (graveyard, moved_cards) in moved_cards_by_graveyard {
        let Ok(mut contents) = zone_contents.get_mut(graveyard) else {
            continue;
        };
        let moved_cards: BTreeSet<Entity> = moved_cards.into_iter().collect();
        let (mut entered_this_frame, already_there): (Vec<Entity>, Vec<Entity>) =
            contents.iter().copied().partition(|card| moved_cards.contains(card));
        entered_this_frame.reverse();
        entered_this_frame.extend(already_there);
        **contents = entered_this_frame;
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::testing::*;
    use crate::{GRAVEYARD, HAND, mana, zones};

    #[test]
    fn a_countered_spell_sits_below_the_spell_that_countered_it() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT, CANCEL]).mana_pool(mana![1, U, U, R]),
            P2,
        ]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        let [cancel] = game.player(p1).cards_in(HAND).exactly::<1>(CANCEL)?;

        game.play([
            p1.cast(bolt).targeting(p2),
            p1.cast(cancel).targeting(bolt).pass_priority(),
            p2.pass_priority(),
        ])?;
        let [cancel_buried] = game.player(p1).cards_in(GRAVEYARD).exactly::<1>(CANCEL)?;
        let [bolt_buried] = game.player(p1).cards_in(GRAVEYARD).exactly::<1>(LIGHTNING_BOLT)?;
        game.play([p1.arrange_graveyard([cancel_buried, bolt_buried])])?;

        let graveyard = game.zone_entity(p1, GRAVEYARD).expect("p1 graveyard");
        let top_first = game.with_query::<&zones::ZoneContents, Vec<Entity>>(|contents| {
            contents.get(graveyard).map(|cards| cards.iter().copied().collect()).unwrap_or_default()
        });
        assert_eq!(
            top_first,
            vec![Entity::from(cancel_buried), Entity::from(bolt_buried)]
        );
        Ok(())
    }
}
