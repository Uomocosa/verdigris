use crate::BATTLEFIELD;
use bevy::prelude::{Entity, MessageWriter, Query};

use crate::cards;
use crate::damage;
use crate::effects;
use crate::rules::helpers;
use crate::zones;

/// 704.5g If a creature has toughness greater than 0, it has damage marked on it, and the total
/// damage marked on it is greater than or equal to its toughness, that creature has been dealt
/// lethal damage and is destroyed. Regeneration can replace this event.
pub fn r704_5g_lethal_damage(
    mut destructions: MessageWriter<damage::Destruction>,
    cards: Query<(
        Entity,
        &cards::CardDefinition,
        &damage::Damage,
        &zones::InZone,
        Option<&effects::PowerToughnessModifier>,
    )>,
    zone_entities: Query<(Entity, &zones::Zone, Option<&cards::Owner>)>,
) {
    for (creature, definition, damage, in_zone, modifier) in cards.iter() {
        if !helpers::is_card_in_zone(**in_zone, BATTLEFIELD, &zone_entities) {
            continue;
        }
        let Some(power_toughness) = helpers::effective_power_toughness(definition, modifier) else {
            continue;
        };
        let marked = helpers::signed_damage(**damage);
        let is_lethal =
            power_toughness.toughness > 0 && marked > 0 && marked >= power_toughness.toughness;
        if !is_lethal {
            continue;
        }
        destructions.write(damage::Destruction(creature));
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
            P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let bolt = game.player(p1).cards(HAND, LIGHTNING_BOLT).only().unwrap();
        let bears = game
            .player(p2)
            .cards(BATTLEFIELD, GRIZZLY_BEARS)
            .only()
            .unwrap();
        game.play([
            p1.cast(bolt).targeting(bears).pass_priority(),
            p2.pass_priority(),
        ])
        .unwrap();
        assert_eq!(game.zone_of(bears), GRAVEYARD);
    }
}
