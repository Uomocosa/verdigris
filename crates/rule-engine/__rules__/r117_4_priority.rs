use bevy::prelude::{Entity, Local, MessageReader, MessageWriter, Query, With};

use crate::actions;
use crate::rules::helpers;
use crate::stack;
use crate::zones;

/// 117.4 If all players pass in succession (that is, if all players pass without taking any
/// actions in between passing), the spell or ability on top of the stack resolves or, if the
/// stack is empty, the phase or step ends.
pub fn r117_4_priority(
    mut actions: MessageReader<actions::PlayerAction>,
    mut resolutions: MessageWriter<stack::Resolution>,
    mut passes: Local<usize>,
    players: Query<Entity, With<zones::Player>>,
    cards: Query<(&zones::InZone, Option<&stack::CastingStage>)>,
    zone_entities: Query<(Entity, &zones::Zone, Option<&zones::Owner>)>,
    stack_contents: Query<&zones::ZoneContents>,
) {
    let player_count = players.iter().count();
    for action in actions.read() {
        if matches!(&action.action, actions::Action::PassPriority) {
            *passes = passes.saturating_add(1);
        } else {
            *passes = 0;
        }
    }
    if *passes < player_count {
        return;
    }
    let Some(stack) = helpers::find_zone(zones::Zone::Stack, None, &zone_entities) else {
        *passes = 0;
        return;
    };
    let Some(top) = stack_contents
        .get(stack)
        .ok()
        .and_then(|contents| contents.last().copied())
    else {
        *passes = 0;
        return;
    };
    let is_still_casting = cards.get(top).is_ok_and(|(_, stage)| stage.is_some());
    if is_still_casting {
        *passes = 0;
        return;
    }
    resolutions.write(stack::Resolution(top));
    *passes = 0;
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
            .mana_pool(testing::Seat::P1, mana![R])
            .build();
        let bolt = game
            .find(game.p1, zones::Zone::Hand, "Lightning Bolt")
            .unwrap();
        game.act(game.p1, actions::Action::Cast(bolt));
        game.act(game.p1, actions::Action::ChooseTargets(vec![game.p2]));
        game.pass_priority_all();
        assert_eq!(game.zone_of(bolt), zones::Zone::Graveyard);
    }
}
