use bevy::prelude::{Commands, Entity, MessageWriter, Query};

use crate::cards;
use crate::rules::helpers;
use crate::zones;

/// 704.5h If a creature has toughness greater than 0, and it's been dealt damage by a source with
/// deathtouch since the last time state-based actions were checked, that creature is destroyed.
/// Regeneration can replace this event.
pub fn r704_5h_deathtouch_damage(
    mut destructions: MessageWriter<zones::Destruction>,
    mut commands: Commands,
    cards: Query<(
        Entity,
        &cards::CardDefinition,
        &zones::DealtByDeathtouch,
        &zones::InZone,
    )>,
    zone_entities: Query<(Entity, &zones::Zone, Option<&zones::Owner>)>,
) {
    for (creature, definition, _, in_zone) in cards.iter() {
        commands
            .entity(creature)
            .remove::<zones::DealtByDeathtouch>();
        if !helpers::is_card_in_zone(**in_zone, zones::Zone::Battlefield, &zone_entities) {
            continue;
        }
        let Some(power_toughness) = definition.power_toughness else {
            continue;
        };
        if power_toughness.toughness > 0 {
            destructions.write(zones::Destruction(creature));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;
    use crate::zones;

    #[test]
    #[ignore = "needs combat: Typhoid Rats deals damage by attacking or blocking"]
    fn test_usage() {
        let game = MockGame::builder()
            .battlefield(Seat::P1, [typhoid_rats()])
            .battlefield(Seat::P2, [grizzly_bears()])
            .build();
        assert!(
            game.find(game.p1, zones::Zone::Battlefield, "Typhoid Rats")
                .is_some()
        );
        assert!(
            game.find(game.p2, zones::Zone::Battlefield, "Grizzly Bears")
                .is_some()
        );
    }

    #[test]
    #[ignore = "needs combat: Typhoid Rats deals damage by attacking or blocking"]
    fn r704_5h_deathtouch_damage_destroys_a_larger_creature() {
        let game = MockGame::builder()
            .battlefield(Seat::P1, [typhoid_rats()])
            .battlefield(Seat::P2, [grizzly_bears()])
            .build();
        assert!(
            game.find(game.p1, zones::Zone::Battlefield, "Typhoid Rats")
                .is_some()
        );
        assert!(
            game.find(game.p2, zones::Zone::Battlefield, "Grizzly Bears")
                .is_some()
        );
    }

    #[test]
    #[ignore = "needs combat: Typhoid Rats deals damage by attacking or blocking"]
    fn r704_5h_deathtouch_damage_does_not_destroy_indestructible() {
        let game = MockGame::builder()
            .battlefield(Seat::P1, [typhoid_rats()])
            .battlefield(Seat::P2, [darksteel_myr()])
            .build();
        assert!(
            game.find(game.p2, zones::Zone::Battlefield, "Darksteel Myr")
                .is_some()
        );
    }

    #[test]
    #[ignore = "needs combat: Typhoid Rats deals damage by attacking or blocking"]
    fn r704_5h_damage_without_deathtouch_does_not_destroy_a_two_two() {
        let game = MockGame::builder()
            .battlefield(Seat::P1, [walking_corpse()])
            .battlefield(Seat::P2, [grizzly_bears()])
            .build();
        assert!(
            game.find(game.p2, zones::Zone::Battlefield, "Grizzly Bears")
                .is_some()
        );
    }
}
