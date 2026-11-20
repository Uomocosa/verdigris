use crate::BATTLEFIELD;
use crate::GRAVEYARD;
use bevy::prelude::{Commands, Entity, Query};

use crate::cards;
use crate::effects;
use crate::rules::helpers;
use crate::zones;

/// 704.5f If a creature has toughness 0 or less, it's put into its owner's
/// graveyard. Regeneration can't replace this event.
pub fn r704_5f_zero_toughness(
    mut commands: Commands,
    cards: Query<(
        Entity,
        &cards::CardDefinition,
        &cards::Owner,
        &zones::InZone,
        Option<&effects::PowerToughnessModifier>,
    )>,
    zone_entities: Query<(Entity, &zones::Zone, Option<&cards::Owner>)>,
) {
    for (creature, definition, owner, in_zone, modifier) in cards.iter() {
        if !helpers::is_card_in_zone(**in_zone, BATTLEFIELD, &zone_entities) {
            continue;
        }
        let Some(power_toughness) = helpers::effective_power_toughness(definition, modifier) else {
            continue;
        };
        if power_toughness.toughness > 0 {
            continue;
        }
        if let Some(graveyard) = helpers::find_zone(GRAVEYARD, Some(**owner), &zone_entities) {
            commands.entity(creature).insert(zones::InZone(graveyard));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mana;
    use crate::testing::*;
    use crate::{BATTLEFIELD, GRAVEYARD, HAND};

    #[test]
    fn test_usage() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([DISFIGURE]).mana_pool(mana![B]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [disfigure] =
            game.player(p1).cards_in(HAND).matching(DISFIGURE).exactly::<1>().unwrap();
        let [bears] =
            game.player(p2).cards_in(BATTLEFIELD).matching(GRIZZLY_BEARS).exactly::<1>().unwrap();
        game.play([
            p1.cast(disfigure).targeting(bears).pass_priority(),
            p2.pass_priority(),
        ])
        .unwrap();
        assert_eq!(game.zone_of(bears), GRAVEYARD);
    }

    #[test]
    fn r704_5f_zero_toughness_puts_a_zero_toughness_creature_in_the_graveyard() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([DISFIGURE]).mana_pool(mana![B]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [disfigure] =
            game.player(p1).cards_in(HAND).matching(DISFIGURE).exactly::<1>().unwrap();
        let [bears] =
            game.player(p2).cards_in(BATTLEFIELD).matching(GRIZZLY_BEARS).exactly::<1>().unwrap();
        game.play([
            p1.cast(disfigure).targeting(bears).pass_priority(),
            p2.pass_priority(),
        ])
        .unwrap();
        assert!(!game.player(p2).cards_in(BATTLEFIELD).has(GRIZZLY_BEARS));
        assert!(game.player(p2).cards_in(GRAVEYARD).has(GRIZZLY_BEARS));
        assert_eq!(game.zone_of(disfigure), GRAVEYARD);
    }
}
