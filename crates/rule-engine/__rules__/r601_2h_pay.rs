use bevy::prelude::{Commands, Entity, MessageWriter, Query};

use crate::actions;
use crate::cards;
use crate::rules::helpers;
use crate::stack;
use crate::zones;

/// 601.2h The player pays the total cost. First, they pay all costs that don't involve random
/// elements or moving objects from the library to a public zone, in any order. ... Partial
/// payments are not allowed. Unpayable costs can't be paid.
pub fn r601_2h_pay(
    mut rejected: MessageWriter<actions::Rejected>,
    mut commands: Commands,
    cards: Query<(
        Entity,
        &cards::CardDefinition,
        &zones::Owner,
        &stack::CastingStage,
    )>,
    mut pools: Query<&mut cards::ManaPool>,
    zone_entities: Query<(Entity, &zones::Zone, Option<&zones::Owner>)>,
) {
    for (spell, definition, owner, stage) in cards.iter() {
        if *stage != stack::CastingStage::AwaitingPayment {
            continue;
        }
        let Ok(mut pool) = pools.get_mut(**owner) else {
            continue;
        };
        if pool.pay(definition.cost) {
            commands.entity(spell).remove::<stack::CastingStage>();
            continue;
        }
        rejected.write(actions::Rejected::CannotPay { spell });
        if let Some(hand) = helpers::find_zone(zones::Zone::Hand, Some(**owner), &zone_entities) {
            commands.entity(spell).insert(zones::InZone(hand));
        }
        commands.entity(spell).remove::<stack::CastingStage>();
    }
}

#[cfg(test)]
mod tests {
    use crate::actions;
    use crate::cards;
    use crate::mana;
    use crate::testing;
    use crate::zones;

    #[test]
    fn test_usage() {
        let mut game = testing::MockGame::builder()
            .hand(testing::Seat::P1, [testing::lightning_bolt()])
            .mana_pool(testing::Seat::P1, mana![R])
            .build();
        let bolt = game
            .find(game.p1, zones::Zone::Hand, "Lightning Bolt")
            .unwrap();
        game.act(game.p1, actions::Action::Cast(bolt));
        game.act(game.p1, actions::Action::ChooseTargets(vec![game.p2]));
        assert!(game.last_rejection().is_none());
        assert_eq!(game.mana_pool(game.p1), cards::ManaPool::default());
    }

    #[test]
    fn test_cannot_cast_without_mana() {
        let mut game = testing::MockGame::builder()
            .hand(testing::Seat::P1, [testing::lightning_bolt()])
            .build();
        let bolt = game
            .find(game.p1, zones::Zone::Hand, "Lightning Bolt")
            .unwrap();
        game.act(game.p1, actions::Action::Cast(bolt));
        game.act(game.p1, actions::Action::ChooseTargets(vec![game.p2]));
        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotPay { spell: bolt })
        );
        assert_eq!(game.zone_of(bolt), zones::Zone::Hand);
    }
}
