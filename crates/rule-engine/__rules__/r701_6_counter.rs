use crate::{GRAVEYARD, STACK};
use bevy::prelude::{Commands, Entity, MessageReader, Query};

use crate::cards;
use crate::rules::helpers;
use crate::stack;
use crate::zones;

/// 701.6a To counter a spell or ability means to cancel it, removing it from the stack. It
/// doesn't resolve and none of its effects occur. A countered spell is put into its owner's
/// graveyard.
pub fn r701_6_counter(
    mut countered: MessageReader<stack::Countered>,
    mut commands: Commands,
    spells: Query<(&cards::Owner, &zones::InZone)>,
    zone_entities: Query<(Entity, &zones::Zone, Option<&cards::Owner>)>,
) {
    for event in countered.read() {
        let spell = **event;
        let Ok((owner, in_zone)) = spells.get(spell) else {
            continue;
        };
        if !helpers::is_card_in_zone(**in_zone, STACK, &zone_entities) {
            continue;
        }
        if let Some(graveyard) = helpers::find_zone(GRAVEYARD, Some(**owner), &zone_entities) {
            commands.entity(spell).insert(zones::InZone(graveyard));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::actions;
    use crate::mana;
    use crate::testing::*;
    use crate::{GRAVEYARD, HAND};

    #[test]
    fn test_usage() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]),
            P2.hand([CANCEL]).mana_pool(mana![1, U, U]),
        ]);
        let bolt = game.player(p1).cards(HAND, LIGHTNING_BOLT).only().unwrap();
        let cancel = game.player(p2).cards(HAND, CANCEL).only().unwrap();

        game.act(p1, actions::Action::Cast(bolt));
        game.act(p1, actions::Action::ChooseTargets(vec![*p2]));
        game.pass_priority();
        game.act(p2, actions::Action::Cast(cancel));
        game.act(p2, actions::Action::ChooseTargets(vec![bolt]));
        game.pass_priority_all();

        assert_eq!(game.life(p2), 20);
        assert_eq!(game.zone_of(bolt), GRAVEYARD);
        assert_eq!(game.zone_of(cancel), GRAVEYARD);
    }

    #[test]
    fn essence_scatter_counters_a_creature_spell() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([GRIZZLY_BEARS]).mana_pool(mana![1, G]),
            P2.hand([ESSENCE_SCATTER]).mana_pool(mana![1, U]),
        ]);
        let bears = game.player(p1).cards(HAND, GRIZZLY_BEARS).only().unwrap();
        let scatter = game.player(p2).cards(HAND, ESSENCE_SCATTER).only().unwrap();

        game.act(p1, actions::Action::Cast(bears));
        game.pass_priority();
        game.act(p2, actions::Action::Cast(scatter));
        game.act(p2, actions::Action::ChooseTargets(vec![bears]));
        game.pass_priority_all();

        assert_eq!(game.zone_of(bears), GRAVEYARD);
        assert_eq!(game.zone_of(scatter), GRAVEYARD);
    }

    #[test]
    fn negate_counters_a_noncreature_spell() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]),
            P2.hand([NEGATE]).mana_pool(mana![1, U]),
        ]);
        let bolt = game.player(p1).cards(HAND, LIGHTNING_BOLT).only().unwrap();
        let negate = game.player(p2).cards(HAND, NEGATE).only().unwrap();

        game.act(p1, actions::Action::Cast(bolt));
        game.act(p1, actions::Action::ChooseTargets(vec![*p2]));
        game.pass_priority();
        game.act(p2, actions::Action::Cast(negate));
        game.act(p2, actions::Action::ChooseTargets(vec![bolt]));
        game.pass_priority_all();

        assert_eq!(game.life(p2), 20);
        assert_eq!(game.zone_of(bolt), GRAVEYARD);
        assert_eq!(game.zone_of(negate), GRAVEYARD);
    }

    #[test]
    fn dispel_can_counter_a_cancel_so_the_outer_bolt_resolves() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT, DISPEL]).mana_pool(mana![R, U]),
            P2.hand([CANCEL]).mana_pool(mana![1, U, U]),
        ]);
        let bolt = game.player(p1).cards(HAND, LIGHTNING_BOLT).only().unwrap();
        let dispel = game.player(p1).cards(HAND, DISPEL).only().unwrap();
        let cancel = game.player(p2).cards(HAND, CANCEL).only().unwrap();

        game.act(p1, actions::Action::Cast(bolt));
        game.act(p1, actions::Action::ChooseTargets(vec![*p2]));
        game.pass_priority();
        game.act(p2, actions::Action::Cast(cancel));
        game.act(p2, actions::Action::ChooseTargets(vec![bolt]));
        game.pass_priority();
        game.act(p1, actions::Action::Cast(dispel));
        game.act(p1, actions::Action::ChooseTargets(vec![cancel]));

        game.pass_priority_all();
        assert_eq!(game.zone_of(cancel), GRAVEYARD);
        assert_eq!(game.life(p2), 20);

        game.pass_priority_all();
        assert_eq!(game.life(p2), 17);
        assert_eq!(game.zone_of(bolt), GRAVEYARD);
        assert_eq!(game.zone_of(dispel), GRAVEYARD);
    }

    #[test]
    fn cancelled_cancel_does_not_stop_the_outer_bolt() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT, CANCEL])
                .mana_pool(mana![1, U, U, R]),
            P2.hand([CANCEL]).mana_pool(mana![1, U, U]),
        ]);
        let bolt = game.player(p1).cards(HAND, LIGHTNING_BOLT).only().unwrap();
        let p1_cancel = game.player(p1).cards(HAND, CANCEL).only().unwrap();
        let p2_cancel = game.player(p2).cards(HAND, CANCEL).only().unwrap();

        game.act(p1, actions::Action::Cast(bolt));
        game.act(p1, actions::Action::ChooseTargets(vec![*p2]));
        game.pass_priority();
        game.act(p2, actions::Action::Cast(p2_cancel));
        game.act(p2, actions::Action::ChooseTargets(vec![bolt]));
        game.pass_priority();
        game.act(p1, actions::Action::Cast(p1_cancel));
        game.act(p1, actions::Action::ChooseTargets(vec![p2_cancel]));

        game.pass_priority_all();
        assert_eq!(game.zone_of(p2_cancel), GRAVEYARD);
        assert_eq!(game.life(p2), 20);

        game.pass_priority_all();
        assert_eq!(game.life(p2), 17);
        assert_eq!(game.zone_of(bolt), GRAVEYARD);
        assert_eq!(game.zone_of(p1_cancel), GRAVEYARD);
    }
}
