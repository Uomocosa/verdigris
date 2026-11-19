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
    use crate::testing::*;
    use crate::zones;

    #[test]
    fn test_usage() {
        let mut game = MockGame::builder()
            .hand(Seat::P1, [lightning_bolt()])
            .hand(Seat::P2, [cancel()])
            .mana_pool(Seat::P1, mana![R])
            .mana_pool(Seat::P2, mana![1, U, U])
            .build();
        let bolt = game
            .find(game.p1, zones::Zone::Hand, "Lightning Bolt")
            .unwrap();
        let cancel = game.find(game.p2, zones::Zone::Hand, "Cancel").unwrap();
        let p2 = game.p2;

        game.act(game.p1, actions::Action::Cast(bolt));
        game.act(game.p1, actions::Action::ChooseTargets(vec![p2]));
        game.pass_priority();
        game.act(p2, actions::Action::Cast(cancel));
        game.act(p2, actions::Action::ChooseTargets(vec![bolt]));
        game.pass_priority_all();

        assert_eq!(game.life(p2), 20);
        assert_eq!(game.zone_of(bolt), zones::Zone::Graveyard);
        assert_eq!(game.zone_of(cancel), zones::Zone::Graveyard);
    }

    #[test]
    fn essence_scatter_counters_a_creature_spell() {
        let mut game = MockGame::builder()
            .hand(Seat::P1, [grizzly_bears()])
            .hand(Seat::P2, [essence_scatter()])
            .mana_pool(Seat::P1, mana![1, G])
            .mana_pool(Seat::P2, mana![1, U])
            .build();
        let bears = game
            .find(game.p1, zones::Zone::Hand, "Grizzly Bears")
            .unwrap();
        let scatter = game
            .find(game.p2, zones::Zone::Hand, "Essence Scatter")
            .unwrap();

        game.act(game.p1, actions::Action::Cast(bears));
        game.pass_priority();
        game.act(game.p2, actions::Action::Cast(scatter));
        game.act(game.p2, actions::Action::ChooseTargets(vec![bears]));
        game.pass_priority_all();

        assert_eq!(game.zone_of(bears), zones::Zone::Graveyard);
        assert_eq!(game.zone_of(scatter), zones::Zone::Graveyard);
    }

    #[test]
    fn negate_counters_a_noncreature_spell() {
        let mut game = MockGame::builder()
            .hand(Seat::P1, [lightning_bolt()])
            .hand(Seat::P2, [negate()])
            .mana_pool(Seat::P1, mana![R])
            .mana_pool(Seat::P2, mana![1, U])
            .build();
        let bolt = game
            .find(game.p1, zones::Zone::Hand, "Lightning Bolt")
            .unwrap();
        let negate = game.find(game.p2, zones::Zone::Hand, "Negate").unwrap();
        let p2 = game.p2;

        game.act(game.p1, actions::Action::Cast(bolt));
        game.act(game.p1, actions::Action::ChooseTargets(vec![p2]));
        game.pass_priority();
        game.act(p2, actions::Action::Cast(negate));
        game.act(p2, actions::Action::ChooseTargets(vec![bolt]));
        game.pass_priority_all();

        assert_eq!(game.life(p2), 20);
        assert_eq!(game.zone_of(bolt), zones::Zone::Graveyard);
        assert_eq!(game.zone_of(negate), zones::Zone::Graveyard);
    }

    #[test]
    fn dispel_can_counter_a_cancel_so_the_outer_bolt_resolves() {
        let mut game = MockGame::builder()
            .hand(Seat::P1, [lightning_bolt(), dispel()])
            .hand(Seat::P2, [cancel()])
            .mana_pool(Seat::P1, mana![R, U])
            .mana_pool(Seat::P2, mana![1, U, U])
            .build();
        let bolt = game
            .find(game.p1, zones::Zone::Hand, "Lightning Bolt")
            .unwrap();
        let dispel = game.find(game.p1, zones::Zone::Hand, "Dispel").unwrap();
        let cancel = game.find(game.p2, zones::Zone::Hand, "Cancel").unwrap();
        let p1 = game.p1;
        let p2 = game.p2;

        game.act(p1, actions::Action::Cast(bolt));
        game.act(p1, actions::Action::ChooseTargets(vec![p2]));
        game.pass_priority();
        game.act(p2, actions::Action::Cast(cancel));
        game.act(p2, actions::Action::ChooseTargets(vec![bolt]));
        game.pass_priority();
        game.act(p1, actions::Action::Cast(dispel));
        game.act(p1, actions::Action::ChooseTargets(vec![cancel]));

        game.pass_priority_all();
        assert_eq!(game.zone_of(cancel), zones::Zone::Graveyard);
        assert_eq!(game.life(p2), 20);

        game.pass_priority_all();
        assert_eq!(game.life(p2), 17);
        assert_eq!(game.zone_of(bolt), zones::Zone::Graveyard);
        assert_eq!(game.zone_of(dispel), zones::Zone::Graveyard);
    }

    #[test]
    fn cancelled_cancel_does_not_stop_the_outer_bolt() {
        let mut game = MockGame::builder()
            .hand(Seat::P1, [lightning_bolt(), cancel()])
            .hand(Seat::P2, [cancel()])
            .mana_pool(Seat::P1, mana![1, U, U, R])
            .mana_pool(Seat::P2, mana![1, U, U])
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
        game.pass_priority();
        game.act(p2, actions::Action::Cast(p2_cancel));
        game.act(p2, actions::Action::ChooseTargets(vec![bolt]));
        game.pass_priority();
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
