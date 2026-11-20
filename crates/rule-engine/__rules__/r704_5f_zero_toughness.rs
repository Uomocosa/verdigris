use crate::BATTLEFIELD;
use crate::GRAVEYARD;
use bevy::prelude::{Commands, Entity, Query};

use crate::cards;
use crate::effects;
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
        if !BATTLEFIELD.contains_card(**in_zone, &zone_entities) {
            continue;
        }
        let Some(power_toughness) = definition.effective_power_toughness(modifier) else {
            continue;
        };
        if power_toughness.toughness > 0 {
            continue;
        }
        if let Some(graveyard) = GRAVEYARD.find_entity(Some(**owner), &zone_entities) {
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
    fn test_usage() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([DISFIGURE]).mana_pool(mana![B]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [disfigure] = game.player(p1).cards_in(HAND).exactly::<1>(DISFIGURE)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.cast(disfigure).targeting(bears).pass_priority(),
            p2.pass_priority(),
        ])?;
        assert_eq!(game.zone_of(bears), GRAVEYARD);
        Ok(())
    }

    #[test]
    fn r704_5f_zero_toughness_puts_a_zero_toughness_creature_in_the_graveyard() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([DISFIGURE]).mana_pool(mana![B]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [disfigure] = game.player(p1).cards_in(HAND).exactly::<1>(DISFIGURE)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.cast(disfigure).targeting(bears).pass_priority(),
            p2.pass_priority(),
        ])?;
        assert!(!game.player(p2).cards_in(BATTLEFIELD).has(GRIZZLY_BEARS));
        assert!(game.player(p2).cards_in(GRAVEYARD).has(GRIZZLY_BEARS));
        assert_eq!(game.zone_of(disfigure), GRAVEYARD);
        Ok(())
    }
}
