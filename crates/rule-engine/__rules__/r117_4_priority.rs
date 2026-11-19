use bevy::prelude::{Entity, MessageWriter, Query, Res, ResMut};

use crate::rules::helpers;
use crate::stack;
use crate::turns;
use crate::zones;

/// 117.4 If all players pass in succession (that is, if all players pass without taking any
/// actions in between passing), the spell or ability on top of the stack resolves or, if the
/// stack is empty, the phase or step ends.
pub fn r117_4_priority(
    mut resolutions: MessageWriter<stack::Resolution>,
    mut priority: ResMut<turns::Priority>,
    turn: Res<turns::Turn>,
    turn_order: Res<turns::TurnOrder>,
    cards: Query<(&zones::InZone, Option<&stack::CastingStage>)>,
    zone_entities: Query<(Entity, &zones::Zone, Option<&zones::Owner>)>,
    stack_contents: Query<&zones::ZoneContents>,
) {
    let turn = turn.into_inner();
    let turn_order = turn_order.into_inner();
    let player_count = u32::try_from(turn_order.len()).unwrap_or(u32::MAX);
    if priority.consecutive_passes < player_count {
        return;
    }
    priority.consecutive_passes = 0;
    let Some(stack) = helpers::find_zone(zones::Zone::Stack, None, &zone_entities) else {
        return;
    };
    let Some(top) = stack_contents
        .get(stack)
        .ok()
        .and_then(|contents| contents.last().copied())
    else {
        return;
    };
    let is_still_casting = cards.get(top).is_ok_and(|(_, stage)| stage.is_some());
    if is_still_casting {
        return;
    }
    priority.player = turn.active;
    resolutions.write(stack::Resolution(top));
}

#[cfg(test)]
mod tests {
    use crate::actions;
    use crate::mana;
    use crate::testing;
    use crate::turns;
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
        game.pass_priority();
        game.pass_priority();
        assert_eq!(game.zone_of(bolt), zones::Zone::Graveyard);
    }

    #[test]
    fn all_players_passing_resolves_one_spell() {
        let mut game = testing::MockGame::builder()
            .hand(
                testing::Seat::P1,
                [testing::lightning_bolt(), testing::lightning_bolt()],
            )
            .mana_pool(testing::Seat::P1, mana![R, R])
            .build();
        let first = game
            .find(game.p1, zones::Zone::Hand, "Lightning Bolt")
            .unwrap();
        game.act(game.p1, actions::Action::Cast(first));
        game.act(game.p1, actions::Action::ChooseTargets(vec![game.p2]));
        let second = game
            .find(game.p1, zones::Zone::Hand, "Lightning Bolt")
            .unwrap();
        game.act(game.p1, actions::Action::Cast(second));
        game.act(game.p1, actions::Action::ChooseTargets(vec![game.p2]));

        game.pass_priority();
        game.pass_priority();

        assert_eq!(game.zone_of(second), zones::Zone::Graveyard);
        assert_eq!(game.zone_of(first), zones::Zone::Stack);
        assert_eq!(game.life(game.p2), 17);
    }

    #[test]
    fn priority_returns_to_the_active_player_after_a_spell_resolves() {
        let mut game = testing::MockGame::builder()
            .hand(testing::Seat::P1, [testing::lightning_bolt()])
            .mana_pool(testing::Seat::P1, mana![R])
            .build();
        let p1 = game.p1;
        let bolt = game.find(p1, zones::Zone::Hand, "Lightning Bolt").unwrap();
        game.act(p1, actions::Action::Cast(bolt));
        game.act(p1, actions::Action::ChooseTargets(vec![game.p2]));

        game.pass_priority();
        assert_ne!(game.app.world().resource::<turns::Priority>().player, p1);
        game.pass_priority();

        let priority = *game.app.world().resource::<turns::Priority>();
        assert_eq!(priority.player, p1);
        assert_eq!(priority.consecutive_passes, 0);
    }
}
