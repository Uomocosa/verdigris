use bevy::prelude::{Commands, Entity, Query};

use crate::cards;
use crate::rules::helpers;
use crate::zones;

/// 704.5g If a creature has toughness greater than 0, it has damage marked on it, and the total
/// damage marked on it is greater than or equal to its toughness, that creature has been dealt
/// lethal damage and is destroyed. Regeneration can replace this event.
pub fn r704_5g_lethal_damage(
    mut commands: Commands,
    cards: Query<(
        Entity,
        &cards::CardDefinition,
        &zones::Damage,
        &zones::Owner,
        &zones::InZone,
    )>,
    zone_entities: Query<(Entity, &zones::Zone, Option<&zones::Owner>)>,
) {
    for (creature, definition, damage, owner, in_zone) in cards.iter() {
        if !helpers::is_card_in_zone(**in_zone, zones::Zone::Battlefield, &zone_entities) {
            continue;
        }
        let Some(power_toughness) = definition.power_toughness else {
            continue;
        };
        let marked = helpers::signed_damage(**damage);
        let is_lethal =
            power_toughness.toughness > 0 && marked > 0 && marked >= power_toughness.toughness;
        if !is_lethal {
            continue;
        }
        if let Some(graveyard) =
            helpers::find_zone(zones::Zone::Graveyard, Some(**owner), &zone_entities)
        {
            commands.entity(creature).insert(zones::InZone(graveyard));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::actions;
    use crate::mana;
    use crate::testing;
    use crate::zones;

    #[test]
    fn test_usage() {
        let mut game = testing::MockGame::builder()
            .hand(testing::Seat::P1, [testing::lightning_bolt()])
            .battlefield(testing::Seat::P2, [testing::grizzly_bears()])
            .mana_pool(testing::Seat::P1, mana![R])
            .build();
        let bolt = game
            .find(game.p1, zones::Zone::Hand, "Lightning Bolt")
            .unwrap();
        let bears = game
            .find(game.p2, zones::Zone::Battlefield, "Grizzly Bears")
            .unwrap();
        game.act(game.p1, actions::Action::Cast(bolt));
        game.act(game.p1, actions::Action::ChooseTargets(vec![bears]));
        game.pass_priority_all();
        assert_eq!(game.zone_of(bears), zones::Zone::Graveyard);
    }
}
