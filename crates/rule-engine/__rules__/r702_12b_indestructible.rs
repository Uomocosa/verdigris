use crate::{BATTLEFIELD, GRAVEYARD};
use bevy::prelude::{Commands, Entity, MessageReader, Query};

use crate::cards;
use crate::damage;
use crate::rules::helpers;
use crate::zones;

/// 702.12b A permanent with indestructible can't be destroyed. Such permanents aren't destroyed by
/// lethal damage, and they ignore the state-based action that checks for lethal damage (see rule
/// 704.5g).
pub fn r702_12b_indestructible(
    mut destructions: MessageReader<damage::Destruction>,
    mut commands: Commands,
    cards: Query<(&cards::CardDefinition, &cards::Owner, &zones::InZone)>,
    zone_entities: Query<(Entity, &zones::Zone, Option<&cards::Owner>)>,
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
        if !helpers::is_card_in_zone(**in_zone, BATTLEFIELD, &zone_entities) {
            continue;
        }
        if let Some(graveyard) = helpers::find_zone(GRAVEYARD, Some(**owner), &zone_entities) {
            commands.entity(permanent).insert(zones::InZone(graveyard));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::actions;
    use crate::damage;
    use crate::mana;
    use crate::testing::*;
    use crate::{BATTLEFIELD, GRAVEYARD, HAND};

    #[test]
    fn test_usage() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([murder()]).mana_pool(mana![1, B, B]),
            P2.battlefield([darksteel_myr()]),
        ]);
        let murder = game.find(p1, HAND, "Murder").unwrap();
        let myr = game.find(p2, BATTLEFIELD, "Darksteel Myr").unwrap();
        game.act(p1, actions::Action::Cast(murder));
        game.act(p1, actions::Action::ChooseTargets(vec![myr]));
        game.pass_priority_all();
        assert_eq!(game.zone_of(myr), BATTLEFIELD);
    }

    #[test]
    fn r702_12b_indestructible_ignores_destroy_effects() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([murder()]).mana_pool(mana![1, B, B]),
            P2.battlefield([darksteel_myr()]),
        ]);
        let murder = game.find(p1, HAND, "Murder").unwrap();
        let myr = game.find(p2, BATTLEFIELD, "Darksteel Myr").unwrap();
        game.act(p1, actions::Action::Cast(murder));
        game.act(p1, actions::Action::ChooseTargets(vec![myr]));
        game.pass_priority_all();
        assert_eq!(game.zone_of(myr), BATTLEFIELD);
        assert_eq!(game.zone_of(murder), GRAVEYARD);
        assert!(game.find(p2, BATTLEFIELD, "Darksteel Myr").is_some());
    }

    #[test]
    fn r702_12b_indestructible_survives_lethal_damage() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([lightning_bolt()]).mana_pool(mana![R]),
            P2.battlefield([darksteel_myr()]),
        ]);
        let bolt = game.find(p1, HAND, "Lightning Bolt").unwrap();
        let myr = game.find(p2, BATTLEFIELD, "Darksteel Myr").unwrap();
        game.act(p1, actions::Action::Cast(bolt));
        game.act(p1, actions::Action::ChooseTargets(vec![myr]));
        game.pass_priority_all();
        assert_eq!(game.zone_of(myr), BATTLEFIELD);
        let damage = game.app.world().get::<damage::Damage>(myr).unwrap();
        assert_eq!(**damage, 3);
    }

    #[test]
    fn r702_12b_indestructible_survives_doom_blade() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([doom_blade()]).mana_pool(mana![1, B]),
            P2.battlefield([darksteel_myr()]),
        ]);
        let blade = game.find(p1, HAND, "Doom Blade").unwrap();
        let myr = game.find(p2, BATTLEFIELD, "Darksteel Myr").unwrap();
        game.act(p1, actions::Action::Cast(blade));
        game.act(p1, actions::Action::ChooseTargets(vec![myr]));
        game.pass_priority_all();
        assert_eq!(game.zone_of(myr), BATTLEFIELD);
        assert_eq!(game.zone_of(blade), GRAVEYARD);
    }
}
