use std::collections::BTreeMap;

use bevy::prelude::{App, Entity};

use crate::RulesPlugin;
use crate::cards;
use crate::damage;
use crate::players;
use crate::testing;
use crate::turns;
use crate::zones;
use crate::{BATTLEFIELD, EXILE, GRAVEYARD, HAND, LIBRARY};

pub fn spawn_game(
    builder: &testing::MockGameBuilder,
    players: &[testing::PlayerSetup],
) -> (testing::MockGame, Vec<Entity>) {
    for (index, setup) in players.iter().enumerate() {
        assert!(
            setup.seat == index.saturating_add(1),
            "MockGame: P{} listed in position {}",
            setup.seat,
            index.saturating_add(1)
        );
    }
    let active_seats: Vec<usize> =
        players.iter().filter_map(|setup| setup.active.then_some(setup.seat)).collect();
    assert!(
        active_seats.len() <= 1,
        "MockGame: P{} and P{} are both marked .active(); mark at most one",
        active_seats.first().copied().unwrap_or_default(),
        active_seats.get(1).copied().unwrap_or_default()
    );

    let mut app = App::new();
    app.add_plugins(RulesPlugin);
    let entities = testing::MockGameBuilder::spawn_players(&mut app, players);

    let zone_maps: Vec<BTreeMap<zones::Zone, Entity>> = entities
        .iter()
        .map(|player| {
            [LIBRARY, HAND, BATTLEFIELD, GRAVEYARD, EXILE]
                .into_iter()
                .map(|zone| {
                    let zone_entity = app.world_mut().spawn((zone, cards::Owner(*player))).id();
                    (zone, zone_entity)
                })
                .collect()
        })
        .collect();
    let stack = app.world_mut().spawn((zones::Zone::Stack,)).id();

    for (index, (player, setup)) in entities.iter().zip(players).enumerate() {
        let Some(player_zones) = zone_maps.get(index) else {
            continue;
        };
        let hand = player_zones.get(&HAND).copied().unwrap_or(Entity::PLACEHOLDER);
        for definition in setup.hand.iter().cloned() {
            let _ = app.world_mut().spawn((definition, zones::InZone(hand), cards::Owner(*player)));
        }

        let battlefield = player_zones.get(&BATTLEFIELD).copied().unwrap_or(Entity::PLACEHOLDER);
        for definition in setup.battlefield.iter().cloned() {
            let _ = app.world_mut().spawn((
                definition,
                zones::InZone(battlefield),
                cards::Owner(*player),
                damage::Damage(0),
            ));
        }
        testing::MockGameBuilder::spawn_tapped_battlefield(&mut app, *player, setup, battlefield);

        let library = player_zones.get(&LIBRARY).copied().unwrap_or(Entity::PLACEHOLDER);
        let library_cards = setup
            .library
            .clone()
            .unwrap_or_else(|| vec![cards::CardDefinition::from(testing::GRIZZLY_BEARS); 12]);
        for definition in library_cards {
            let _ =
                app.world_mut().spawn((definition, zones::InZone(library), cards::Owner(*player)));
        }
    }

    let active = entities
        .get(players.iter().position(|setup| setup.active).unwrap_or(0))
        .copied()
        .unwrap_or(Entity::PLACEHOLDER);
    let active = players::PlayerId(active);
    app.insert_resource(turns::Turn {
        number: builder.turn_number,
        active,
        step: builder.step,
    });
    app.insert_resource(turns::Priority {
        player: active,
        consecutive_passes: 0,
    });
    app.insert_resource(turns::TurnOrder::from(
        entities.iter().copied().map(players::PlayerId).collect::<Vec<_>>(),
    ));

    let game = testing::MockGame {
        players: entities.clone(),
        zones: zone_maps,
        stack,
        app,
    };
    (game, entities)
}

#[cfg(test)]
mod tests {
    use crate::players;
    use crate::testing::*;
    use crate::turns;
    use crate::zones;

    use super::spawn_game;

    #[test]
    fn test_usage() {
        let builder = MockGameBuilder::default();
        let (game, entities) = spawn_game(&builder, &[P1, P2.active()]);

        assert_eq!(entities.len(), 2);
        assert_eq!(game.life(entities[0]), 20);
        let turn = game.turn();
        assert_eq!(turn.active, players::PlayerId(entities[1]));
        assert_eq!(game.priority_player(), players::PlayerId(entities[1]));
        assert_eq!(game.app.world().resource::<turns::TurnOrder>().len(), 2);
        assert_eq!(
            game.get::<zones::Zone>(game.stack),
            Some(zones::Zone::Stack)
        );
    }
}
