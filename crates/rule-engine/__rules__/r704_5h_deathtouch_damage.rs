use crate::BATTLEFIELD;
use bevy::prelude::{Commands, Entity, MessageWriter, Query};

use crate::cards;
use crate::damage;
use crate::rules::helpers;
use crate::zones;

/// 704.5h If a creature has toughness greater than 0, and it's been dealt damage by a source with
/// deathtouch since the last time state-based actions were checked, that creature is destroyed.
/// Regeneration can replace this event.
pub fn r704_5h_deathtouch_damage(
    mut destructions: MessageWriter<damage::Destruction>,
    mut commands: Commands,
    cards: Query<(
        Entity,
        &cards::CardDefinition,
        &damage::DealtByDeathtouch,
        &zones::InZone,
    )>,
    zone_entities: Query<(Entity, &zones::Zone, Option<&cards::Owner>)>,
) {
    for (creature, definition, _, in_zone) in cards.iter() {
        commands
            .entity(creature)
            .remove::<damage::DealtByDeathtouch>();
        if !helpers::is_card_in_zone(**in_zone, BATTLEFIELD, &zone_entities) {
            continue;
        }
        let Some(power_toughness) = definition.power_toughness else {
            continue;
        };
        if power_toughness.toughness > 0 {
            destructions.write(damage::Destruction(creature));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::testing::*;

    #[test]
    #[ignore = "needs combat: Typhoid Rats deals damage by attacking or blocking"]
    fn test_usage() {
        let (game, [p1, p2]) = MockGame::new([
            P1.battlefield([TYPHOID_RATS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        assert!(game.player(p1).cards_in(BATTLEFIELD).has(TYPHOID_RATS));
        assert!(game.player(p2).cards_in(BATTLEFIELD).has(GRIZZLY_BEARS));
    }

    #[test]
    #[ignore = "needs combat: Typhoid Rats deals damage by attacking or blocking"]
    fn r704_5h_deathtouch_damage_destroys_a_larger_creature() {
        let (game, [p1, p2]) = MockGame::new([
            P1.battlefield([TYPHOID_RATS]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        assert!(game.player(p1).cards_in(BATTLEFIELD).has(TYPHOID_RATS));
        assert!(game.player(p2).cards_in(BATTLEFIELD).has(GRIZZLY_BEARS));
    }

    #[test]
    #[ignore = "needs combat: Typhoid Rats deals damage by attacking or blocking"]
    fn r704_5h_deathtouch_damage_does_not_destroy_indestructible() {
        let (game, [_, p2]) = MockGame::new([
            P1.battlefield([TYPHOID_RATS]),
            P2.battlefield([DARKSTEEL_MYR]),
        ]);
        assert!(game.player(p2).cards_in(BATTLEFIELD).has(DARKSTEEL_MYR));
    }

    #[test]
    #[ignore = "needs combat: Typhoid Rats deals damage by attacking or blocking"]
    fn r704_5h_damage_without_deathtouch_does_not_destroy_a_two_two() {
        let (game, [_, p2]) = MockGame::new([
            P1.battlefield([WALKING_CORPSE]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        assert!(game.player(p2).cards_in(BATTLEFIELD).has(GRIZZLY_BEARS));
    }
}
