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
    use crate::testing;
    use crate::zones;

    #[test]
    fn test_usage() {
        let mut game = testing::MockGame::builder()
            .battlefield(testing::Seat::P1, [testing::typhoid_rats()])
            .battlefield(testing::Seat::P2, [testing::grizzly_bears()])
            .build();
        let rats = game
            .find(game.p1, zones::Zone::Battlefield, "Typhoid Rats")
            .unwrap();
        let bears = game
            .find(game.p2, zones::Zone::Battlefield, "Grizzly Bears")
            .unwrap();
        let _ = game.app.world_mut().write_message(zones::DamageDealt {
            source: rats,
            target: bears,
            amount: 1,
        });
        game.app.update();
        assert_eq!(game.zone_of(bears), zones::Zone::Graveyard);
    }

    #[test]
    fn r704_5h_deathtouch_damage_destroys_a_larger_creature() {
        let mut game = testing::MockGame::builder()
            .battlefield(testing::Seat::P1, [testing::typhoid_rats()])
            .battlefield(testing::Seat::P2, [testing::grizzly_bears()])
            .build();
        let rats = game
            .find(game.p1, zones::Zone::Battlefield, "Typhoid Rats")
            .unwrap();
        let bears = game
            .find(game.p2, zones::Zone::Battlefield, "Grizzly Bears")
            .unwrap();
        let _ = game.app.world_mut().write_message(zones::DamageDealt {
            source: rats,
            target: bears,
            amount: 1,
        });
        game.app.update();
        assert_eq!(game.zone_of(bears), zones::Zone::Graveyard);
        assert_eq!(game.zone_of(rats), zones::Zone::Battlefield);
    }

    #[test]
    fn r704_5h_deathtouch_damage_does_not_destroy_indestructible() {
        let mut game = testing::MockGame::builder()
            .battlefield(testing::Seat::P1, [testing::typhoid_rats()])
            .battlefield(testing::Seat::P2, [testing::darksteel_myr()])
            .build();
        let rats = game
            .find(game.p1, zones::Zone::Battlefield, "Typhoid Rats")
            .unwrap();
        let myr = game
            .find(game.p2, zones::Zone::Battlefield, "Darksteel Myr")
            .unwrap();
        let _ = game.app.world_mut().write_message(zones::DamageDealt {
            source: rats,
            target: myr,
            amount: 1,
        });
        game.app.update();
        assert_eq!(game.zone_of(myr), zones::Zone::Battlefield);
        let damage = game.app.world().get::<zones::Damage>(myr).unwrap();
        assert_eq!(**damage, 1);
    }

    #[test]
    fn r704_5h_damage_without_deathtouch_does_not_destroy_a_two_two() {
        let mut game = testing::MockGame::builder()
            .battlefield(testing::Seat::P1, [testing::walking_corpse()])
            .battlefield(testing::Seat::P2, [testing::grizzly_bears()])
            .build();
        let source = game
            .find(game.p1, zones::Zone::Battlefield, "Walking Corpse")
            .unwrap();
        let bears = game
            .find(game.p2, zones::Zone::Battlefield, "Grizzly Bears")
            .unwrap();
        let _ = game.app.world_mut().write_message(zones::DamageDealt {
            source,
            target: bears,
            amount: 1,
        });
        game.app.update();
        assert_eq!(game.zone_of(bears), zones::Zone::Battlefield);
        let damage = game.app.world().get::<zones::Damage>(bears).unwrap();
        assert_eq!(**damage, 1);
    }
}
