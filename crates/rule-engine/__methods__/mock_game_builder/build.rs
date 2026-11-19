use std::collections::BTreeMap;

use bevy::prelude::{App, Entity};

use crate::RulesPlugin;
use crate::cards;
use crate::damage;
use crate::testing;
use crate::turns;
use crate::zones;

const SEATS: [testing::Seat; 8] = [
    testing::Seat::P1,
    testing::Seat::P2,
    testing::Seat::P3,
    testing::Seat::P4,
    testing::Seat::P5,
    testing::Seat::P6,
    testing::Seat::P7,
    testing::Seat::P8,
];

pub fn build(builder: &testing::MockGameBuilder) -> testing::MockGame {
    let mut app = App::new();
    app.add_plugins(RulesPlugin);

    let players: Vec<Entity> = builder.spawn_players(&mut app, &SEATS);

    let mut zones_by_seat: BTreeMap<(testing::Seat, zones::Zone), Entity> = BTreeMap::new();
    for (seat, player) in SEATS.iter().zip(&players) {
        for zone in [
            zones::Zone::Library,
            zones::Zone::Hand,
            zones::Zone::Battlefield,
            zones::Zone::Graveyard,
            zones::Zone::Exile,
        ] {
            let zone_entity = app.world_mut().spawn((zone, cards::Owner(*player))).id();
            zones_by_seat.insert((*seat, zone), zone_entity);
        }
    }
    let _ = app.world_mut().spawn((zones::Zone::Stack,)).id();
    for (seat, player) in SEATS.iter().zip(&players) {
        let hand = zones_by_seat
            .get(&(*seat, zones::Zone::Hand))
            .copied()
            .unwrap_or(Entity::PLACEHOLDER);
        for definition in builder.hands.get(seat).into_iter().flatten().cloned() {
            let _ = app
                .world_mut()
                .spawn((definition, zones::InZone(hand), cards::Owner(*player)));
        }

        let battlefield = zones_by_seat
            .get(&(*seat, zones::Zone::Battlefield))
            .copied()
            .unwrap_or(Entity::PLACEHOLDER);
        for definition in builder
            .battlefields
            .get(seat)
            .into_iter()
            .flatten()
            .cloned()
        {
            let _ = app.world_mut().spawn((
                definition,
                zones::InZone(battlefield),
                cards::Owner(*player),
                damage::Damage(0),
            ));
        }

        let library_zone = zones_by_seat
            .get(&(*seat, zones::Zone::Library))
            .copied()
            .unwrap_or(Entity::PLACEHOLDER);
        let library = builder
            .libraries
            .get(seat)
            .cloned()
            .unwrap_or_else(|| vec![testing::grizzly_bears(); 12]);
        for definition in library {
            let _ = app.world_mut().spawn((
                definition,
                zones::InZone(library_zone),
                cards::Owner(*player),
            ));
        }

        let pool = builder.mana_pools.get(seat).copied().unwrap_or_default();
        let _ = app.world_mut().entity_mut(*player).insert(pool);
    }

    let active = SEATS
        .iter()
        .position(|seat| *seat == builder.active)
        .and_then(|index| players.get(index))
        .copied()
        .unwrap_or(Entity::PLACEHOLDER);
    app.insert_resource(turns::Turn {
        number: builder.turn_number,
        active,
        step: builder.step,
    });
    app.insert_resource(turns::Priority {
        player: active,
        consecutive_passes: 0,
    });
    app.insert_resource(turns::TurnOrder::from(players.clone()));

    let p1 = players.first().copied().unwrap_or(Entity::PLACEHOLDER);
    let p2 = players.get(1).copied().unwrap_or(Entity::PLACEHOLDER);
    testing::MockGame {
        p1,
        p2,
        players,
        app,
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;
    use crate::zones;

    use super::build;

    #[test]
    fn test_usage() {
        let game = build(&MockGameBuilder::default());
        assert_eq!(game.life(game.p1), 20);
        assert!(
            game.find(game.p1, zones::Zone::Library, "Grizzly Bears")
                .is_some()
        );
    }
}
