use bevy::prelude::{Commands, Entity, MessageReader, Query};

use crate::cards;
use crate::rules::helpers;
use crate::zones;

/// 702.12b A permanent with indestructible can't be destroyed. Such permanents aren't destroyed by
/// lethal damage, and they ignore the state-based action that checks for lethal damage (see rule
/// 704.5g).
pub fn r702_12b_indestructible(
    mut destructions: MessageReader<zones::Destruction>,
    mut commands: Commands,
    cards: Query<(&cards::CardDefinition, &zones::Owner, &zones::InZone)>,
    zone_entities: Query<(Entity, &zones::Zone, Option<&zones::Owner>)>,
) {
    for destruction in destructions.read() {
        let permanent = **destruction;
        let Ok((definition, owner, in_zone)) = cards.get(permanent) else {
            continue;
        };
        if definition
            .keywords
            .contains(&cards::Keyword::Indestructible)
        {
            continue;
        }
        if !helpers::is_card_in_zone(**in_zone, zones::Zone::Battlefield, &zone_entities) {
            continue;
        }
        if let Some(graveyard) =
            helpers::find_zone(zones::Zone::Graveyard, Some(**owner), &zone_entities)
        {
            commands.entity(permanent).insert(zones::InZone(graveyard));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::actions;
    use crate::mana;
    use crate::testing::*;
    use crate::zones;

    #[test]
    fn test_usage() {
        let mut game = MockGame::builder()
            .hand(Seat::P1, [murder()])
            .battlefield(Seat::P2, [darksteel_myr()])
            .mana_pool(Seat::P1, mana![1, B, B])
            .build();
        let murder = game.find(game.p1, zones::Zone::Hand, "Murder").unwrap();
        let myr = game
            .find(game.p2, zones::Zone::Battlefield, "Darksteel Myr")
            .unwrap();
        game.act(game.p1, actions::Action::Cast(murder));
        game.act(game.p1, actions::Action::ChooseTargets(vec![myr]));
        game.pass_priority_all();
        assert_eq!(game.zone_of(myr), zones::Zone::Battlefield);
    }

    #[test]
    fn r702_12b_indestructible_ignores_destroy_effects() {
        let mut game = MockGame::builder()
            .hand(Seat::P1, [murder()])
            .battlefield(Seat::P2, [darksteel_myr()])
            .mana_pool(Seat::P1, mana![1, B, B])
            .build();
        let murder = game.find(game.p1, zones::Zone::Hand, "Murder").unwrap();
        let myr = game
            .find(game.p2, zones::Zone::Battlefield, "Darksteel Myr")
            .unwrap();
        game.act(game.p1, actions::Action::Cast(murder));
        game.act(game.p1, actions::Action::ChooseTargets(vec![myr]));
        game.pass_priority_all();
        assert_eq!(game.zone_of(myr), zones::Zone::Battlefield);
        assert_eq!(game.zone_of(murder), zones::Zone::Graveyard);
        assert!(
            game.find(game.p2, zones::Zone::Battlefield, "Darksteel Myr")
                .is_some()
        );
    }

    #[test]
    fn r702_12b_indestructible_survives_lethal_damage() {
        let mut game = MockGame::builder()
            .hand(Seat::P1, [lightning_bolt()])
            .battlefield(Seat::P2, [darksteel_myr()])
            .mana_pool(Seat::P1, mana![R])
            .build();
        let bolt = game
            .find(game.p1, zones::Zone::Hand, "Lightning Bolt")
            .unwrap();
        let myr = game
            .find(game.p2, zones::Zone::Battlefield, "Darksteel Myr")
            .unwrap();
        game.act(game.p1, actions::Action::Cast(bolt));
        game.act(game.p1, actions::Action::ChooseTargets(vec![myr]));
        game.pass_priority_all();
        assert_eq!(game.zone_of(myr), zones::Zone::Battlefield);
        let damage = game.app.world().get::<zones::Damage>(myr).unwrap();
        assert_eq!(**damage, 3);
    }

    #[test]
    fn r702_12b_indestructible_survives_doom_blade() {
        let mut game = MockGame::builder()
            .hand(Seat::P1, [doom_blade()])
            .battlefield(Seat::P2, [darksteel_myr()])
            .mana_pool(Seat::P1, mana![1, B])
            .build();
        let blade = game.find(game.p1, zones::Zone::Hand, "Doom Blade").unwrap();
        let myr = game
            .find(game.p2, zones::Zone::Battlefield, "Darksteel Myr")
            .unwrap();
        game.act(game.p1, actions::Action::Cast(blade));
        game.act(game.p1, actions::Action::ChooseTargets(vec![myr]));
        game.pass_priority_all();
        assert_eq!(game.zone_of(myr), zones::Zone::Battlefield);
        assert_eq!(game.zone_of(blade), zones::Zone::Graveyard);
    }
}
