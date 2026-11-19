use bevy::prelude::{Commands, Entity, MessageReader, Query};

use crate::rules::helpers;
use crate::stack;
use crate::zones;

/// 701.6a To counter a spell or ability means to cancel it, removing it from the stack. It
/// doesn't resolve and none of its effects occur. A countered spell is put into its owner's
/// graveyard.
pub fn r701_6_counter(
    mut countered: MessageReader<stack::Countered>,
    mut commands: Commands,
    spells: Query<(&zones::Owner, &zones::InZone)>,
    zone_entities: Query<(Entity, &zones::Zone, Option<&zones::Owner>)>,
) {
    for event in countered.read() {
        let spell = **event;
        let Ok((owner, in_zone)) = spells.get(spell) else {
            continue;
        };
        if !helpers::is_card_in_zone(**in_zone, zones::Zone::Stack, &zone_entities) {
            continue;
        }
        if let Some(graveyard) =
            helpers::find_zone(zones::Zone::Graveyard, Some(**owner), &zone_entities)
        {
            commands.entity(spell).insert(zones::InZone(graveyard));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::actions;
    use crate::mana;
    use crate::stack;
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
        let _ = game.app.world_mut().write_message(stack::Countered(bolt));
        game.app.update();
        assert_eq!(game.zone_of(bolt), zones::Zone::Graveyard);
    }

    #[test]
    fn cancel_counters_a_target_spell() {
        let mut game = testing::MockGame::builder()
            .hand(testing::Seat::P1, [testing::lightning_bolt()])
            .hand(testing::Seat::P2, [testing::cancel()])
            .mana_pool(testing::Seat::P1, mana![R])
            .mana_pool(testing::Seat::P2, mana![1, U, U])
            .build();
        let bolt = game
            .find(game.p1, zones::Zone::Hand, "Lightning Bolt")
            .unwrap();
        let cancel = game.find(game.p2, zones::Zone::Hand, "Cancel").unwrap();
        let p2 = game.p2;

        game.act(game.p1, actions::Action::Cast(bolt));
        game.act(game.p1, actions::Action::ChooseTargets(vec![p2]));
        game.give_priority(p2);
        game.act(p2, actions::Action::Cast(cancel));
        game.act(p2, actions::Action::ChooseTargets(vec![bolt]));
        game.pass_priority_all();

        assert_eq!(game.life(p2), 20);
        assert_eq!(game.zone_of(bolt), zones::Zone::Graveyard);
        assert_eq!(game.zone_of(cancel), zones::Zone::Graveyard);
    }

    #[test]
    fn cancelled_cancel_does_not_stop_the_outer_bolt() {
        let mut game = testing::MockGame::builder()
            .hand(
                testing::Seat::P1,
                [testing::lightning_bolt(), testing::cancel()],
            )
            .hand(testing::Seat::P2, [testing::cancel()])
            .mana_pool(testing::Seat::P1, mana![1, U, U, R])
            .mana_pool(testing::Seat::P2, mana![1, U, U])
            .build();
        let bolt = game
            .find(game.p1, zones::Zone::Hand, "Lightning Bolt")
            .unwrap();
        let p1_cancel = game.find(game.p1, zones::Zone::Hand, "Cancel").unwrap();
        let p2_cancel = game.find(game.p2, zones::Zone::Hand, "Cancel").unwrap();
        let p1 = game.p1;
        let p2 = game.p2;

        game.act(p1, actions::Action::Cast(bolt));
        game.act(p1, actions::Action::ChooseTargets(vec![p2]));
        game.give_priority(p2);
        game.act(p2, actions::Action::Cast(p2_cancel));
        game.act(p2, actions::Action::ChooseTargets(vec![bolt]));
        game.give_priority(p1);
        game.act(p1, actions::Action::Cast(p1_cancel));
        game.act(p1, actions::Action::ChooseTargets(vec![p2_cancel]));

        game.pass_priority_all();
        assert_eq!(game.zone_of(p2_cancel), zones::Zone::Graveyard);
        assert_eq!(game.life(p2), 20);

        game.pass_priority_all();
        assert_eq!(game.life(p2), 17);
        assert_eq!(game.zone_of(bolt), zones::Zone::Graveyard);
        assert_eq!(game.zone_of(p1_cancel), zones::Zone::Graveyard);
    }
}
